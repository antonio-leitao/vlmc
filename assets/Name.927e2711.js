import{S as z,i as A,s as B,e as _,a as S,t as b,b as p,c as m,d as q,n as j,f as o}from"./index.bbeac449.js";function w(a){let l=a[1].last+"",f;return{c(){f=b(l)},m(t,i){p(t,f,i)},p(t,i){i&2&&l!==(l=t[1].last+"")&&q(f,l)},d(t){t&&o(f)}}}function D(a){let l,f,t,i,u,c=a[1].first+"",k,H,d,v,r,N,C,s=a[1].last&&w(a);return{c(){l=_("h2"),l.textContent="Hi there!",f=S(),t=_("p"),i=b(`route props:
  `),u=_("b"),k=b(c),H=S(),d=_("b"),s&&s.c(),v=S(),r=_("p"),N=b("static props: "),C=b(a[0])},m(e,n){p(e,l,n),p(e,f,n),p(e,t,n),m(t,i),m(t,u),m(u,k),m(t,H),m(t,d),s&&s.m(d,null),p(e,v,n),p(e,r,n),m(r,N),m(r,C)},p(e,[n]){n&2&&c!==(c=e[1].first+"")&&q(k,c),e[1].last?s?s.p(e,n):(s=w(e),s.c(),s.m(d,null)):s&&(s.d(1),s=null),n&1&&q(C,e[0])},i:j,o:j,d(e){e&&o(l),e&&o(f),e&&o(t),s&&s.d(),e&&o(v),e&&o(r)}}}function E(a,l,f){let{num:t}=l,{params:i={}}=l;return a.$$set=u=>{"num"in u&&f(0,t=u.num),"params"in u&&f(1,i=u.params)},[t,i]}class G extends z{constructor(l){super(),A(this,l,E,D,B,{num:0,params:1})}}export{G as default};
