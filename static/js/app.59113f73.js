(function(t){function n(n){for(var r,o,u=n[0],i=n[1],s=n[2],f=0,d=[];f<u.length;f++)o=u[f],c[o]&&d.push(c[o][0]),c[o]=0;for(r in i)Object.prototype.hasOwnProperty.call(i,r)&&(t[r]=i[r]);l&&l(n);while(d.length)d.shift()();return a.push.apply(a,s||[]),e()}function e(){for(var t,n=0;n<a.length;n++){for(var e=a[n],r=!0,o=1;o<e.length;o++){var u=e[o];0!==c[u]&&(r=!1)}r&&(a.splice(n--,1),t=i(i.s=e[0]))}return t}var r={},o={app:0},c={app:0},a=[];function u(t){return i.p+"js/"+({}[t]||t)+"."+{"chunk-07904ab9":"5d7c7b7d","chunk-1be64b06":"05a6e53c","chunk-1ffc33e0":"c0dbb9e9","chunk-2d0e5e97":"60f6120f","chunk-32a200b3":"d51fe526","chunk-3a97e886":"4fb8c8d5","chunk-46abd30c":"93a006a6","chunk-737f4ed6":"823e478b","chunk-7ab45635":"6edc3025","chunk-8ad230a6":"659437a5","chunk-b412702a":"ce074ba7","chunk-13300983":"eb120160","chunk-2a7ae24d":"13ade982","chunk-486a8857":"7ca35ebb","chunk-530f6138":"f0556e4b","chunk-dc00a0a6":"2782f9b6","chunk-eb3436f0":"55947adb","chunk-c5ebe236":"87c78396"}[t]+".js"}function i(n){if(r[n])return r[n].exports;var e=r[n]={i:n,l:!1,exports:{}};return t[n].call(e.exports,e,e.exports,i),e.l=!0,e.exports}i.e=function(t){var n=[],e={"chunk-07904ab9":1,"chunk-1be64b06":1,"chunk-1ffc33e0":1,"chunk-32a200b3":1,"chunk-3a97e886":1,"chunk-46abd30c":1,"chunk-737f4ed6":1,"chunk-7ab45635":1,"chunk-8ad230a6":1,"chunk-13300983":1,"chunk-2a7ae24d":1,"chunk-486a8857":1,"chunk-530f6138":1,"chunk-dc00a0a6":1,"chunk-eb3436f0":1,"chunk-c5ebe236":1};o[t]?n.push(o[t]):0!==o[t]&&e[t]&&n.push(o[t]=new Promise(function(n,e){for(var r="css/"+({}[t]||t)+"."+{"chunk-07904ab9":"cc20eddf","chunk-1be64b06":"3da6bb7f","chunk-1ffc33e0":"605bebab","chunk-2d0e5e97":"31d6cfe0","chunk-32a200b3":"1de2c35b","chunk-3a97e886":"f9dcd98d","chunk-46abd30c":"4e6131eb","chunk-737f4ed6":"adb94cf5","chunk-7ab45635":"aec1fbdf","chunk-8ad230a6":"568b3681","chunk-b412702a":"31d6cfe0","chunk-13300983":"5bf466c3","chunk-2a7ae24d":"96d16136","chunk-486a8857":"37f48c39","chunk-530f6138":"7a7ddd8e","chunk-dc00a0a6":"2c6e9d77","chunk-eb3436f0":"5a7842fa","chunk-c5ebe236":"46b8ed9f"}[t]+".css",c=i.p+r,a=document.getElementsByTagName("link"),u=0;u<a.length;u++){var s=a[u],f=s.getAttribute("data-href")||s.getAttribute("href");if("stylesheet"===s.rel&&(f===r||f===c))return n()}var d=document.getElementsByTagName("style");for(u=0;u<d.length;u++){s=d[u],f=s.getAttribute("data-href");if(f===r||f===c)return n()}var l=document.createElement("link");l.rel="stylesheet",l.type="text/css",l.onload=n,l.onerror=function(n){var r=n&&n.target&&n.target.src||c,a=new Error("Loading CSS chunk "+t+" failed.\n("+r+")");a.request=r,delete o[t],l.parentNode.removeChild(l),e(a)},l.href=c;var h=document.getElementsByTagName("head")[0];h.appendChild(l)}).then(function(){o[t]=0}));var r=c[t];if(0!==r)if(r)n.push(r[2]);else{var a=new Promise(function(n,e){r=c[t]=[n,e]});n.push(r[2]=a);var s,f=document.createElement("script");f.charset="utf-8",f.timeout=120,i.nc&&f.setAttribute("nonce",i.nc),f.src=u(t),s=function(n){f.onerror=f.onload=null,clearTimeout(d);var e=c[t];if(0!==e){if(e){var r=n&&("load"===n.type?"missing":n.type),o=n&&n.target&&n.target.src,a=new Error("Loading chunk "+t+" failed.\n("+r+": "+o+")");a.type=r,a.request=o,e[1](a)}c[t]=void 0}};var d=setTimeout(function(){s({type:"timeout",target:f})},12e4);f.onerror=f.onload=s,document.head.appendChild(f)}return Promise.all(n)},i.m=t,i.c=r,i.d=function(t,n,e){i.o(t,n)||Object.defineProperty(t,n,{enumerable:!0,get:e})},i.r=function(t){"undefined"!==typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(t,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(t,"__esModule",{value:!0})},i.t=function(t,n){if(1&n&&(t=i(t)),8&n)return t;if(4&n&&"object"===typeof t&&t&&t.__esModule)return t;var e=Object.create(null);if(i.r(e),Object.defineProperty(e,"default",{enumerable:!0,value:t}),2&n&&"string"!=typeof t)for(var r in t)i.d(e,r,function(n){return t[n]}.bind(null,r));return e},i.n=function(t){var n=t&&t.__esModule?function(){return t["default"]}:function(){return t};return i.d(n,"a",n),n},i.o=function(t,n){return Object.prototype.hasOwnProperty.call(t,n)},i.p="/",i.oe=function(t){throw console.error(t),t};var s=window["webpackJsonp"]=window["webpackJsonp"]||[],f=s.push.bind(s);s.push=n,s=s.slice();for(var d=0;d<s.length;d++)n(s[d]);var l=f;a.push([0,"chunk-vendors"]),e()})({0:function(t,n,e){t.exports=e("56d7")},"034f":function(t,n,e){"use strict";var r=e("85ec"),o=e.n(r);o.a},"0a5a":function(t,n,e){"use strict";e.d(n,"c",function(){return u}),e.d(n,"g",function(){return i}),e.d(n,"e",function(){return s}),e.d(n,"b",function(){return f}),e.d(n,"f",function(){return d}),e.d(n,"d",function(){return l}),e.d(n,"a",function(){return h});var r=e("a78e"),o=e.n(r),c="No-0Is-3SeS-8Nek-0oTr",a="Yt-1IT-7nEdIr-2Sa";function u(){return o.a.get(c)}function i(t){var n=arguments.length>1&&void 0!==arguments[1]?arguments[1]:0;return o.a.set(c,t,{expires:n})}function s(){return o.a.remove(c)}function f(){return o.a.get(a)}function d(t){var n=arguments.length>1&&void 0!==arguments[1]?arguments[1]:0;return o.a.set(a,t,{expires:n})}function l(){return o.a.remove(a)}function h(){var t=u();return!!t}},"1db0":function(t,n,e){"use strict";var r=e("d3c1"),o=e.n(r);o.a},"41cb":function(t,n,e){"use strict";var r=e("2b0e"),o=e("8c4f"),c=function(){var t=this,n=t.$createElement,e=t._self._c||n;return e("div",{staticClass:"progress",style:{width:t.percent+"%",height:t.height,"background-color":t.canSuccess?t.color:t.failedColor,opacity:t.show?1:0}})},a=[],u={data:function(){return{percent:0,show:!1,canSuccess:!0,duration:3e3,height:"2px",color:"#ffca2b",failedColor:"#ff0000"}},methods:{start:function(){var t=this;return this.show=!0,this.canSuccess=!0,this._timer&&(clearInterval(this._timer),this.percent=0),this._cut=1e4/Math.floor(this.duration),this._timer=setInterval(function(){t.increase(t._cut*Math.random()),t.percent>95&&t.finish()},100),this},set:function(t){return this.show=!0,this.canSuccess=!0,this.percent=Math.floor(t),this},get:function(){return Math.floor(this.percent)},increase:function(t){return this.percent=this.percent+Math.floor(t),this},decrease:function(t){return this.percent=this.percent-Math.floor(t),this},finish:function(){return this.percent=100,this.hide(),this},pause:function(){return clearInterval(this._timer),this},hide:function(){var t=this;return clearInterval(this._timer),this._timer=null,setTimeout(function(){t.show=!1,t.$nextTick(function(){setTimeout(function(){t.percent=0},200)})},500),this},fail:function(){return this.canSuccess=!1,this}}},i=u,s=(e("1db0"),e("2877")),f=Object(s["a"])(i,c,a,!1,null,"5b964bfc",null),d=f.exports,l=e("0a5a"),h=function(){return Promise.all([e.e("chunk-b412702a"),e.e("chunk-486a8857")]).then(e.bind(null,"bb51"))},p=function(){return e.e("chunk-8ad230a6").then(e.bind(null,"5a00"))},m=function(){return e.e("chunk-1ffc33e0").then(e.bind(null,"7f71"))},b=function(){return e.e("chunk-3a97e886").then(e.bind(null,"c66d"))},g=function(t,n,r){return function(){return Promise.all([e.e("chunk-b412702a"),e.e("chunk-2a7ae24d")]).then(e.bind(null,"277e")).then(function(e){return e.default(t,n,r)})}},v=function(t,n,r){return function(){return e.e("chunk-7ab45635").then(e.bind(null,"5cb4")).then(function(e){return e.default(t,n,r)})}},k=function(){return e.e("chunk-c5ebe236").then(e.bind(null,"2b47"))},w=function(){return Promise.all([e.e("chunk-b412702a"),e.e("chunk-eb3436f0")]).then(e.bind(null,"b97a"))},y=function(){return Promise.all([e.e("chunk-b412702a"),e.e("chunk-13300983")]).then(e.bind(null,"c4d5"))},T=function(){return e.e("chunk-1be64b06").then(e.bind(null,"5bdd"))},_=function(){return e.e("chunk-07904ab9").then(e.bind(null,"f87b"))},x=function(){return e.e("chunk-32a200b3").then(e.bind(null,"4f89"))},E=function(){return e.e("chunk-737f4ed6").then(e.bind(null,"635f"))},S=function(){return e.e("chunk-46abd30c").then(e.bind(null,"4ae6"))},O=function(){return Promise.all([e.e("chunk-b412702a"),e.e("chunk-530f6138")]).then(e.bind(null,"2c02"))},j=function(){return Promise.all([e.e("chunk-b412702a"),e.e("chunk-dc00a0a6")]).then(e.bind(null,"ad42"))},N=function(){return e.e("chunk-2d0e5e97").then(e.bind(null,"9703"))};r["default"].use(o["a"]);var I=new o["a"]({mode:"history",scrollBehavior:function(){return{y:0}},routes:[{path:"/",name:"Home",component:h},{path:"/register",component:p,name:"Register"},{path:"/login",component:m,name:"Login"},{path:"/p/:id",component:b,meta:{auth:!0},children:[{path:"",name:"defaultProfile",redirect:"doing"},{path:"created",name:"CreatedRuts",component:g("user","create"),meta:{auth:!0}},{path:"star",name:"StarRuts",component:g("user","star"),meta:{auth:!0}},{path:"todo",name:"Todos",component:v("user","todo"),meta:{auth:!0}},{path:"doing",name:"Doings",component:v("user","doing"),meta:{auth:!0}},{path:"done",name:"Dones",component:v("user","done"),meta:{auth:!0}}]},{path:"/updateuser/:id",name:"UpdateUser",component:k,meta:{auth:!0}},{path:"/r/:id",name:"Rutview",component:w},{path:"/rforum/:id",name:"RutForum",component:y},{path:"/new",name:"NewRut",component:T,meta:{auth:!0}},{path:"/update/r/:id",name:"UpdateRut",component:_,meta:{auth:!0}},{path:"/collect/:id",name:"AddItem",component:x,meta:{auth:!0}},{path:"/submit",name:"NewItem",component:E,meta:{auth:!0}},{path:"/item/:id",name:"Itemview",component:O},{path:"/update/item/:id",name:"UpdateItem",component:S,meta:{auth:!0}},{path:"/tag/:id",component:j,children:[{path:"",name:"defaultTagView",redirect:"r"},{path:"r",name:"TagRuts",component:g("tag")}]},{path:"/404",component:N,name:"NotFound",hidden:!0},{path:"*",hidden:!0,redirect:{path:"/404"}}]}),R=r["default"].prototype.$bar=new r["default"](d).$mount();document.body.appendChild(R.$el),I.beforeEach(function(t,n,e){if(R.start(),t.meta.auth){var r=Object(l["c"])();r?e():e({path:"/login",query:{redirect:t.fullPath}})}else e()}),I.afterEach(function(){R.finish()});n["a"]=I},4360:function(t,n,e){"use strict";var r=e("5176"),o=e.n(r),c=e("795b"),a=e.n(c),u=e("2b0e"),i=e("2f62"),s={actID:function(t){return t.actID},actUser:function(t){return t.actUser},indexRuts:function(t){return t.rut.indexRuts},ruts:function(t){return t.rut.ruts}},f=s,d=(e("ac6a"),e("0a0d")),l=e.n(d),h=e("d722"),p={indexRuts:[],ruts:{}},m={getRut:function(t,n){var e=t.commit,r=t.state,o=r.ruts[n],c=l()();return new a.a(function(t,r){o&&o.id==n&&c-o.lastUpdate<3e5?(console.log("no need re-fetch"),t(o)):Object(h["m"])(n).then(function(n){var r=n.data.rut;e("SET_RUT",{rut:r}),t(r)}).catch(function(t){r(t)})})},getIndexRuts:function(t){var n=t.commit;return new a.a(function(t,e){Object(h["j"])().then(function(e){var r=e.data.ruts;n("SET_RUTS",r,!0),t(r)}).catch(function(t){e(t)})})}},b={SET_RUTS:function(t,n){var e=arguments.length>2&&void 0!==arguments[2]&&arguments[2];e&&(t.indexRuts=n),n.forEach(function(n){var e=n.id,r=t.ruts[e];if(!r){var c=o()({lastUpdate:l()()},n);u["default"].set(t.ruts,e,c)}})},SET_RUT:function(t,n){var e=n.rut,r=e.id,c=o()({lastUpdate:l()()},e);u["default"].set(t.ruts,r,c)},RENEW_RUT:function(t,n){try{t.ruts[n.rutid][n.ref]=n[n.ref]}catch(e){return}}},g={state:p,actions:m,mutations:b},v={items:{}},k={getItem:function(t,n){var e=t.state,r=t.commit,o=e.items[n],c=l()();return new a.a(function(t,e){o&&o.id===n&&c-o.lastUpdate<3e5?t(o):Object(h["k"])(n).then(function(n){var e=n.data.item;r("SET_ITEM",{item:e}),t(e)}).catch(function(t){e(t)})})}},w={SET_ITEM:function(t,n){var e=n.item,r=o()({lastUpdate:l()()},e);u["default"].set(t.items,r.id,r)},RENEW_ITEMS:function(t,n){t.items[n.itemid][n.ref]=n[n.ref]}},y={state:v,actions:k,mutations:w},T=e("0a5a"),_=void 0;u["default"].use(i["a"]);n["a"]=new i["a"].Store({state:{token:Object(T["c"])(),authed:Object(T["a"])(),actID:Object(T["b"])(),actUser:{}},mutations:{SET_TOKEN:function(t,n){var e=n.token,r=n.uname;t.token=e,t.actID=r,t.authed=Boolean(r),Object(T["g"])(e,n.exp),Object(T["f"])(r,n.exp)},DEL_TOKEN:function(t){t.token="",t.authed=!1,Object(T["e"])(),Object(T["d"])()},SET_INFO:function(t,n){t.actUser=n}},actions:{login:function(t,n){var e=t.commit;return new a.a(function(t,r){Object(h["u"])(n).then(function(n){var r=n.data;if(200===r.status){var c=o()(r,{uname:r.user.uname});e("SET_TOKEN",c),t(n)}else _.$message("Something Failed")}).catch(function(t){r(t)})})}},getters:f,modules:{rut:g,item:y}})},"56d7":function(t,n,e){"use strict";e.r(n);var r=e("a4bb"),o=e.n(r),c=(e("ac6a"),e("cadf"),e("551c"),e("f751"),e("097d"),e("2b0e")),a=function(){var t=this,n=t.$createElement,e=t._self._c||n;return e("div",{attrs:{id:"app"}},[e("header",{staticClass:"header"},[e("nav",{staticClass:"nav-menu"},[e("router-link",{attrs:{to:"/"}},[e("small",{staticStyle:{color:"darkorange","font-size":"1.2em","letter-spacing":"0.005em"}},[t._v("\n            RutHub"),e("sup",{staticStyle:{"font-size":"0.5em",color:"grey"}},[t._v(" alpha")])])]),e("div",{staticClass:"right-menu"},[t.authed?e("div",[e("el-dropdown",[e("el-button",{attrs:{type:"success",size:"small"}},[e("i",{staticClass:"el-icon-menu"}),e("i",{staticClass:"el-icon-arrow-down el-icon--right"})]),e("el-dropdown-menu",{attrs:{slot:"dropdown"},slot:"dropdown"},[e("el-dropdown-item",[e("router-link",{attrs:{to:"/new"}},[e("b",{staticStyle:{color:"orange"}},[t._v("New")])])],1),e("el-dropdown-item",[e("router-link",{attrs:{to:"/p/"+t.currID}},[t._v("Profile")])],1),e("el-dropdown-item",{attrs:{divided:""}},[e("el-button",{attrs:{type:"text"},on:{click:t.onLogout}},[t._v("Log out")])],1)],1)],1)],1):e("div",[e("el-button",{attrs:{type:"text"},on:{click:function(n){t.toLogin=!0}}},[t._v("\n            Log in\n          ")])],1),e("el-dialog",{staticClass:"loginDialog",attrs:{visible:t.toLogin,width:"450px"},on:{"update:visible":function(n){t.toLogin=n}}},[e("login-form",{attrs:{next:"current"},on:{close:function(n){t.toLogin=!1}}})],1)],1)],1)]),e("div",{staticClass:"view"},[e("router-view")],1),e("footer",{staticClass:"footer"},[e("div",{staticClass:"bottom"},[t._v("\n      ©RutHub - Since 2018\n      | "),e("router-link",{attrs:{to:"/submit"}},[t._v("Submit")]),t._v("\n      | "),e("router-link",{attrs:{to:"/new"}},[t._v("Create")]),t._v("\n      | "),e("router-link",{attrs:{to:"#"}},[t._v("About")])],1)])])},u=[],i=e("adb1"),s={name:"app",components:{LoginForm:i["a"]},data:function(){return{toLogin:!1}},computed:{authed:function(){return this.$store.state.authed},currID:function(){return this.$store.state.actID}},methods:{onLogout:function(){this.$store.commit("DEL_TOKEN")}}},f=s,d=(e("034f"),e("2877")),l=Object(d["a"])(f,a,u,!1,null,null,null),h=l.exports,p=e("41cb"),m=e("4360"),b=e("9483");function g(t){var n=t.$options.title;if(n)return"function"===typeof n?n.call(t):n}function v(t){var n=g(t);n&&(document.title="".concat(n," - RutHub"))}Object(b["a"])("".concat("/","service-worker.js"),{ready:function(){console.log("App is being served from cache by a service worker.\nFor more details, visit https://goo.gl/AFskqB")},registered:function(){console.log("Service worker has been registered.")},cached:function(){console.log("Content has been cached for offline use.")},updatefound:function(){console.log("New content is downloading.")},updated:function(){console.log("New content is available; please refresh.")},offline:function(){console.log("No internet connection found. App is running in offline mode.")},error:function(t){console.error("Error during service worker registration:",t)}});var k={beforeUpdate:function(){v(this)},mounted:function(){v(this)}},w=k,y=e("5ad4"),T=e("bb71"),_=e("7496"),x=e("4bd4"),E=e("2677"),S=e("a844"),O=(e("bf40"),e("0fae"),e("b2d6")),j=e.n(O),N=e("4897"),I=e.n(N),R=e("5c96");c["default"].use(T["a"],{components:{VApp:_["a"],VForm:x["a"],VTextField:E["a"],VTextarea:S["a"]}}),I.a.use(j.a),c["default"].use(R["Dialog"]),c["default"].use(R["Dropdown"]),c["default"].use(R["DropdownMenu"]),c["default"].use(R["DropdownItem"]),c["default"].use(R["Button"]),c["default"].use(R["ButtonGroup"]),c["default"].use(R["Select"]),c["default"].use(R["Option"]),c["default"].use(R["Input"]),c["default"].prototype.$message=R["Message"],o()(y).forEach(function(t){c["default"].filter(t,y[t])}),c["default"].mixin(w),c["default"].config.productionTip=!1,new c["default"]({router:p["a"],store:m["a"],render:function(t){return t(h)}}).$mount("#app")},"5ad4":function(t,n,e){"use strict";e.r(n),e.d(n,"pluralise",function(){return u}),e.d(n,"timeAgo",function(){return i}),e.d(n,"timeGap",function(){return s}),e.d(n,"toLocal",function(){return f}),e.d(n,"toMDY",function(){return l}),e.d(n,"showLess",function(){return h}),e.d(n,"host",function(){return p});e("28a5");var r=e("fc16"),o=e.n(r),c=e("0a0d"),a=e.n(c);e("c5f6"),e("a481");function u(t,n){return t+" "+n+(t<=1?"":"s")}function i(t){var n=!(arguments.length>1&&void 0!==arguments[1])||arguments[1],e=n?t.replace(/\s+/g,"T").concat("Z"):t,r=new Date(e),c=r.getTime(),i=0,s=Number(a()())/1e3+i-Number(c)/1e3;return s<3600?o()(~~(s/60),0)?"just now":u(~~(s/60),"minute")+" ago":s<86400?u(~~(s/3600),"hour")+" ago":s<2592e3?u(~~(s/86400),"day")+" ago":s<31104e3?u(~~(s/2592e3),"month")+" ago":u(~~(s/31104e3),"year")+" ago"}function s(t){var n=!(arguments.length>1&&void 0!==arguments[1])||arguments[1],e=n?t.replace(/\s+/g,"T").concat("Z"):t,r=new Date(e),o=r.getTime(),c=Number(a()())/1e3-Number(o)/1e3,i=u(~~(Math.abs(c)/86400),"day");return c<0?i+" Left":"⚠ "+i+" Past"}function f(t){var n=!(arguments.length>1&&void 0!==arguments[1])||arguments[1],e=n?t.replace(/\s+/g,"T").concat("Z"):t;return t?new Date(e).toLocaleString():t}var d={1:"Jan",2:"Feb",3:"Mar",4:"Apr",5:"May",6:"Jun",7:"Jul",8:"Aug",9:"Sep",10:"Oct",11:"Nov",12:"Dec"};function l(t){var n=!(arguments.length>1&&void 0!==arguments[1])||arguments[1];if(!t)return t;var e=n?t.replace(/\s+/g,"T").concat("Z"):t;return t=new Date(e),"".concat(d[t.getMonth()+1]," ").concat(t.getDate(),",").concat(t.getFullYear())}function h(t){var n=arguments.length>1&&void 0!==arguments[1]?arguments[1]:155,e=!(arguments.length>2&&void 0!==arguments[2])||arguments[2];if(!t)return"";if(t.length>n&&e){var r=t.substring(0,n),o=r.lastIndexOf("<a"),c=r.lastIndexOf("</a>"),a=o>c?o:n;return r=r.substring(0,a)+" ...",r}return t}function p(t){var n=t.replace(/^https?:\/\//,"").replace(/\/.*$/,""),e=n.split(".").slice(-3);return"www"===e[0]&&e.shift(),e.join(".")}},"6c53":function(t,n,e){"use strict";var r=e("c2ba"),o=e.n(r);o.a},"85ec":function(t,n,e){},adb1:function(t,n,e){"use strict";var r=function(){var t=this,n=t.$createElement,e=t._self._c||n;return e("div",{staticClass:"login-view"},[e("v-form",{ref:"form",staticClass:"login-form"},[e("v-text-field",{attrs:{label:"Username",rules:t.inRule},model:{value:t.uname,callback:function(n){t.uname=n},expression:"uname"}}),e("v-text-field",{attrs:{label:"Password",type:"password",rules:t.inRule},model:{value:t.password,callback:function(n){t.password=n},expression:"password"}}),e("el-button",{staticClass:"blockbtn",attrs:{type:"primary",size:"small"},on:{click:t.onLogin}},[t._v("Log in\n    ")])],1),e("el-button",{attrs:{type:"text"},on:{click:function(n){return t.toNext("/register")}}},[t._v("\n    No Account? Sign Up\n  ")])],1)},o=[],c={name:"login-form",props:["next"],data:function(){return{uname:"",password:"",inRule:[function(t){return!!t||"required"}]}},methods:{onLogin:function(){var t=this;if(this.$refs.form.validate()){var n={uname:this.uname,password:this.password};this.$store.dispatch("login",n).then(function(){var n=t.$route.path,e="current"===t.next&&"/login"!==n?t.$route.fullPath:t.$route.query.redirect||"/";t.$router.push(e),t.$emit("close")})}else console.log("Error")},toNext:function(t){this.$router.push(t),this.$emit("close")}}},a=c,u=(e("6c53"),e("2877")),i=Object(u["a"])(a,r,o,!1,null,"e6c45ba4",null);n["a"]=i.exports},c2ba:function(t,n,e){},d3c1:function(t,n,e){},d722:function(t,n,e){"use strict";e.d(n,"a",function(){return p}),e.d(n,"v",function(){return b}),e.d(n,"u",function(){return g}),e.d(n,"q",function(){return v}),e.d(n,"E",function(){return k}),e.d(n,"b",function(){return w}),e.d(n,"t",function(){return y}),e.d(n,"C",function(){return T}),e.d(n,"z",function(){return _}),e.d(n,"x",function(){return E}),e.d(n,"d",function(){return x}),e.d(n,"j",function(){return j}),e.d(n,"m",function(){return S}),e.d(n,"n",function(){return O}),e.d(n,"f",function(){return N}),e.d(n,"s",function(){return I}),e.d(n,"k",function(){return R}),e.d(n,"w",function(){return $}),e.d(n,"c",function(){return D}),e.d(n,"h",function(){return C}),e.d(n,"A",function(){return P}),e.d(n,"g",function(){return U}),e.d(n,"l",function(){return L}),e.d(n,"B",function(){return M}),e.d(n,"o",function(){return A}),e.d(n,"p",function(){return F}),e.d(n,"D",function(){return q}),e.d(n,"e",function(){return B}),e.d(n,"y",function(){return z}),e.d(n,"r",function(){return J}),e.d(n,"i",function(){return K});var r=e("bd86"),o=e("5176"),c=e.n(o),a=(e("a481"),e("795b")),u=e.n(a),i=e("2b0e"),s=e("bc3a"),f=e.n(s),d=e("0a5a"),l=e("41cb"),h=e("4360");f.a.interceptors.request.use(function(t){var n=Object(d["c"])();return n&&(t.headers.Authorization=n),t},function(t){return u.a.reject(t)}),f.a.interceptors.response.use(function(t){return t},function(t){if(t.response)switch(t.response.status){case 401:h["a"].commit("DEL_TOKEN"),"/login"!==l["a"].currentRoute.path&&(l["a"].push({path:"/login",query:{redirect:l["a"].currentRoute.fullPath}}),alert("Access Denied, Need To Log in"));break;case 403:alert("Forbidden");break;case 404:alert("Not Found"),l["a"].replace({path:"/404"});break;case 418:alert("Eureka! 42");break;case 500:alert("InternalError"),l["a"].replace({path:"/"});break;default:alert("Something Failed: "+t.response.statusText)}return u.a.reject(t)}),i["default"].prototype.$http=f.a;var p="http://127.0.0.1:8083/api",m=function(t){var n=arguments.length>1&&void 0!==arguments[1]?arguments[1]:{},e=arguments.length>2&&void 0!==arguments[2]?arguments[2]:"get",o=~["get","head","delete"].indexOf(e)?"params":"data";return f()(c()({url:t,method:e},Object(r["a"])({},o,n))).then(function(t){return t})},b=function(t){return m("".concat(p,"/signup"),t,"post")},g=function(t){return m("".concat(p,"/signin"),t,"post")},v=function(t,n){return m("".concat(p,"/users/").concat(t),n)},k=function(t,n){return m("".concat(p,"/users/").concat(t),n,"post")},w=function(t,n){return m("".concat(p,"/users/").concat(t),n,"put")},y=function(t){return m("".concat(p,"/ruts"),t,"post")},T=function(t,n){return m("".concat(p,"/ruts/").concat(t),n,"post")},_=function(t,n,e){return m("".concat(p,"/tagr/").concat(t,"/").concat(n),e,"post")},x=function(t,n){return m("".concat(p,"/ifstarrut/").concat(t),n)},E=function(t){var n=arguments.length>1&&void 0!==arguments[1]?arguments[1]:0,e=arguments.length>2&&void 0!==arguments[2]?arguments[2]:"Love",r=arguments.length>3&&void 0!==arguments[3]?arguments[3]:{};return m("".concat(p,"/starrut/").concat(t,"/").concat(n,"/").concat(e),r)},S=function(t){return m("".concat(p,"/ruts/").concat(t))},O=function(t,n){var e=arguments.length>2&&void 0!==arguments[2]?arguments[2]:1,r=arguments.length>3&&void 0!==arguments[3]?arguments[3]:0;return m("".concat(p,"/ruts/").concat(t,"/").concat(n,"?page=").concat(e,"&flag=").concat(r))},j=function(){return O("index","index")},N=function(t,n){return m("".concat(p,"/collectitem/").concat(t),n,"post")},I=function(t){return m("".concat(p,"/items"),t,"post")},R=function(t,n){return m("".concat(p,"/items/").concat(t),n)},L=function(t,n){var e=arguments.length>2&&void 0!==arguments[2]?arguments[2]:"done",r=arguments.length>3&&void 0!==arguments[3]?arguments[3]:1,o=arguments.length>4&&void 0!==arguments[4]?arguments[4]:{};return m("".concat(p,"/items/").concat(t,"/").concat(n,"?flag=").concat(e,"&page=").concat(r),o)},D=function(t,n){return m("".concat(p,"/itemflag/").concat(t),n)},$=function(t){var n=arguments.length>1&&void 0!==arguments[1]?arguments[1]:"todo",e=arguments.length>2&&void 0!==arguments[2]?arguments[2]:1,r=arguments.length>3&&void 0!==arguments[3]?arguments[3]:"Love";return m("".concat(p,"/staritem/").concat(t,"/").concat(n,"/").concat(e,"/").concat(r))},C=function(t,n,e){return m("".concat(p,"/collects/").concat(t,"/").concat(n),e)},P=function(t,n){return m("".concat(p,"/collects/").concat(t),n,"put")},U=function(t,n){return m("".concat(p,"/collects/").concat(t),n,"delete")},M=function(t,n){return m("".concat(p,"/items/").concat(t),n,"post")},A=function(t,n){return m("".concat(p,"/tags/").concat(t),n)},F=function(t,n,e){return m("".concat(p,"/tags/").concat(t,"/").concat(n),e)},q=function(t,n){return m("".concat(p,"/tags/").concat(t),n,"put")},B=function(t,n){return m("".concat(p,"/ifstartag/").concat(t),n)},z=function(t){var n=arguments.length>1&&void 0!==arguments[1]?arguments[1]:0,e=arguments.length>2&&void 0!==arguments[2]?arguments[2]:"Love",r=arguments.length>3&&void 0!==arguments[3]?arguments[3]:{};return m("".concat(p,"/startag/").concat(t,"/").concat(n,"/").concat(e),r)},J=function(t){return m("".concat(p,"/etcs"),t,"post")},K=function(t,n){var e=arguments.length>2&&void 0!==arguments[2]?arguments[2]:1,r=arguments.length>3&&void 0!==arguments[3]?arguments[3]:{};return m("".concat(p,"/etcs/").concat(t,"/").concat(n,"?page=").concat(e),r)}}});
//# sourceMappingURL=app.59113f73.js.map