using Microsoft.Extensions.Options;
using Microsoft.Extensions.Http;
using System.Net;
using System.Security.Cryptography;
using System.Text.Json;

namespace GnxApp.Monitor;

public sealed class MonitorOptions
{
    public string AppUrl { get; set; } = "https://app.gnx";
    public int IntervalSeconds { get; set; } = 300;
    public int RequestTimeoutSeconds { get; set; } = 15;
    public string? ManifestPath { get; set; }
    public string HashAlgorithm { get; set; } = "SHA256";
}

public sealed class MonitorWorker(
    IHttpClientFactory clients,
    IOptions<MonitorOptions> options,
    ILogger<MonitorWorker> logger) : BackgroundService
{
    private readonly MonitorOptions settings = options.Value;

    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        // Let the Windows Service host report Running before the first network check.
        await Task.Yield();
        using var timer = new PeriodicTimer(TimeSpan.FromSeconds(Math.Clamp(settings.IntervalSeconds, 10, 86400)));
        await CheckOnceAsync(stoppingToken);
        while (await timer.WaitForNextTickAsync(stoppingToken))
            await CheckOnceAsync(stoppingToken);
    }

    private async Task CheckOnceAsync(CancellationToken cancellationToken)
    {
        await CheckUrlAsync(cancellationToken);
        VerifyManifest();
    }

    private async Task CheckUrlAsync(CancellationToken cancellationToken)
    {
        if (!Uri.TryCreate(settings.AppUrl, UriKind.Absolute, out var uri) || uri.Scheme is not ("https" or "http"))
        {
            logger.LogError("Monitor:AppUrl is not a valid HTTP(S) URL: {Url}", settings.AppUrl);
            return;
        }

        try
        {
            using var request = new HttpRequestMessage(HttpMethod.Get, uri);
            using var timeout = CancellationTokenSource.CreateLinkedTokenSource(cancellationToken);
            timeout.CancelAfter(TimeSpan.FromSeconds(Math.Clamp(settings.RequestTimeoutSeconds, 1, 300)));
            using var response = await clients.CreateClient("public-check").SendAsync(request, HttpCompletionOption.ResponseHeadersRead, timeout.Token);
            if (response.StatusCode is >= HttpStatusCode.OK and < HttpStatusCode.MultipleChoices)
                logger.LogInformation("Public URL available: {Url} ({StatusCode})", uri, (int)response.StatusCode);
            else
                logger.LogWarning("Public URL returned {StatusCode}: {Url}", (int)response.StatusCode, uri);
        }
        catch (OperationCanceledException) when (!cancellationToken.IsCancellationRequested)
        {
            logger.LogWarning("Public URL check timed out: {Url}", uri);
        }
        catch (Exception ex)
        {
            logger.LogWarning(ex, "Public URL check failed: {Url}", uri);
        }
    }

    private void VerifyManifest()
    {
        if (string.IsNullOrWhiteSpace(settings.ManifestPath))
        {
            logger.LogDebug("No local install manifest configured; skipping hash verification.");
            return;
        }

        var manifestPath = Path.GetFullPath(settings.ManifestPath, AppContext.BaseDirectory);
        if (!File.Exists(manifestPath))
        {
            logger.LogWarning("Configured install manifest does not exist: {Path}", manifestPath);
            return;
        }

        try
        {
            using var document = JsonDocument.Parse(File.ReadAllText(manifestPath));
            var root = document.RootElement;
            var baseDirectory = root.TryGetProperty("baseDirectory", out var baseElement) && baseElement.ValueKind == JsonValueKind.String
                ? Path.GetFullPath(baseElement.GetString()!, Path.GetDirectoryName(manifestPath)!)
                : Path.GetDirectoryName(manifestPath)!;
            if (!root.TryGetProperty("files", out var files) || files.ValueKind != JsonValueKind.Object)
            {
                logger.LogError("Install manifest has no object-valued 'files' property: {Path}", manifestPath);
                return;
            }

            var mismatches = 0;
            foreach (var entry in files.EnumerateObject())
            {
                var filePath = Path.GetFullPath(entry.Name, baseDirectory);
                if (!File.Exists(filePath))
                {
                    logger.LogError("Manifest file is missing: {Path}", filePath);
                    mismatches++;
                    continue;
                }
                using var algorithm = CreateHashAlgorithm(settings.HashAlgorithm);
                using var stream = File.OpenRead(filePath);
                var actual = Convert.ToHexString(algorithm.ComputeHash(stream));
                if (!actual.Equals(entry.Value.GetString(), StringComparison.OrdinalIgnoreCase))
                {
                    logger.LogError("Manifest hash mismatch: {Path}", filePath);
                    mismatches++;
                }
            }
            logger.LogInformation("Install manifest verified: {Count} file(s), {Mismatches} mismatch(es)", files.EnumerateObject().Count(), mismatches);
        }
        catch (Exception ex)
        {
            logger.LogError(ex, "Could not verify install manifest: {Path}", manifestPath);
        }
    }

    private static HashAlgorithm CreateHashAlgorithm(string name) => name.ToUpperInvariant() switch
    {
        "SHA512" => SHA512.Create(),
        "SHA384" => SHA384.Create(),
        "SHA256" => SHA256.Create(),
        "SHA1" => SHA1.Create(),
        _ => SHA256.Create()
    };
}

public static class Program
{
    public static async Task Main(string[] args)
    {
        var builder = Host.CreateApplicationBuilder(args);
        builder.Services.AddWindowsService(service => service.ServiceName = "GnX App Monitor");
        builder.Services.AddHttpClient("public-check", client => client.DefaultRequestHeaders.UserAgent.ParseAdd("GnXAppMonitor/1.0"));
        builder.Services.Configure<MonitorOptions>(builder.Configuration.GetSection("Monitor"));
        builder.Services.AddHostedService<MonitorWorker>();
        await builder.Build().RunAsync();
    }
}
