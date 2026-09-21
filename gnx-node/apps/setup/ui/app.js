const status=document.querySelector('#status'), detail=document.querySelector('#detail');
const phases=[...document.querySelectorAll('#steps li')];
function render(p){status.textContent=p.message||'Estado del nodo';detail.textContent=p.errorCode?`Código: ${p.errorCode}`:'';phases.forEach((el,i)=>el.className=p.percent!=null&&i*20<p.percent?'done':'');if(p.requiresRestart)document.querySelector('#restart').hidden=false;}
document.querySelector('#provision').addEventListener('click',()=>{render({message:'Solicitud enviada al agente host local…',percent:10});document.querySelector('#provision').hidden=true;});
document.querySelector('#retry').addEventListener('click',()=>render({message:'Reintentando desde el último checkpoint…',percent:0}));
window.addEventListener('load',()=>render({message:'Listo para preparar GnX Node.',percent:0}));
