(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["chunk-bb6391f2"],{"0d83":function(t,e,i){"use strict";var n=function(){var t=this,e=t.$createElement,i=t._self._c||e;return i("div",["0"!==t.action?i("b",[t._v(t._s(t.action.toUpperCase())+": "+t._s(t.totalCount))]):t._e(),i("div",{staticClass:"rut-list"},t._l(t.ruts,function(t){return i("rut-sum",{key:t.id,attrs:{rut:t}})}),1),t.hasMore?i("div",[i("el-button",{attrs:{size:"mini",disabled:!t.hasMore},on:{click:t.loadMoreRut}},[t._v("\n      Show More\n    ")])],1):t._e()])},r=[],s=i("75fc"),o=i("1199"),a=i("d722"),c={name:"rut-list",props:{per:String,action:{type:String,default:"0"},id:String},components:{RutSum:o["a"]},data:function(){return{perid:"",totalCount:0,ruts:[],paging:1}},computed:{hasMore:function(){return this.ruts.length<this.totalCount}},methods:{loadRuts:function(){var t=this,e=this.perid=this.id?this.id:this.$route.params.id;Object(a["n"])(this.per,e,this.paging,this.action).then(function(e){t.ruts=e.data.ruts,t.$store.commit("SET_RUTS",t.ruts),t.totalCount=e.data.count,console.log(e.data.count)})},loadMoreRut:function(){var t=this;Object(a["n"])(this.per,this.perid,this.paging+1,this.action).then(function(e){var i,n=e.data.ruts;t.$store.commit("SET_RUTS",n),(i=t.ruts).push.apply(i,Object(s["a"])(n)),t.paging+=1})}},created:function(){this.loadRuts()}},u=c,l=(i("22b3"),i("2877")),d=Object(l["a"])(u,n,r,!1,null,"8856c02c",null);e["a"]=d.exports},1199:function(t,e,i){"use strict";var n=function(){var t=this,e=t.$createElement,i=t._self._c||e;return i("div",{staticClass:"rut-sum"},[i("span",{staticClass:"title"},[t.rut.url?[t._v("\n      "+t._s(t.rut.title)+"\n      "),i("span",{staticClass:"host"},[i("a",{attrs:{href:t.rut.url,target:"_blank",rel:"nofollow noopener noreferrer"}},[t._v("\n          ("+t._s(t._f("host")(t.rut.url))+")\n        ")])])]:[i("router-link",{attrs:{to:"/r/"+t.rut.id}},[t._v("\n        "+t._s(t.rut.title)+"\n      ")])]],2),i("router-link",{attrs:{to:t.to_url}},[i("span",[i("img",{staticClass:"cover",attrs:{src:t.rut.logo,referrerPolicy:"no-referrer"}})]),i("div",{staticClass:"content",domProps:{innerHTML:t._s(t.content)}}),i("span",{staticClass:"meta"},[t._v("\n      including "+t._s(t._f("pluralise")(t.rut.item_count,"item"))+"  \n    ")])])],1)},r=[],s=i("5ad4"),o=i("e6d6"),a={name:"rut-sum",props:["rut"],computed:{content:function(){var t=Object(o["a"])(this.rut.content);return Object(s["showLess"])(t)},to_url:function(){return this.rut.content?"/r/"+this.rut.id:"/rforum/"+this.rut.id}}},c=a,u=(i("a05d"),i("2877")),l=Object(u["a"])(c,n,r,!1,null,"18a3e056",null);e["a"]=l.exports},"1af6":function(t,e,i){var n=i("63b6");n(n.S,"Array",{isArray:i("9003")})},"20fd":function(t,e,i){"use strict";var n=i("d9f6"),r=i("aebd");t.exports=function(t,e,i){e in t?n.f(t,e,r(0,i)):t[e]=i}},"22b3":function(t,e,i){"use strict";var n=i("bf69"),r=i.n(n);r.a},"2c02":function(t,e,i){"use strict";i.r(e);var n=function(){var t=this,e=t.$createElement,i=t._self._c||e;return t.itemTitle?i("div",{staticClass:"item-page"},[i("div",{staticClass:"item-main"},[i("item-sum",{key:t.item.id,attrs:{item:t.item,out:!0}}),i("div",[i("b",[t._v("More Details")]),i("span",{staticStyle:{float:"right"}},[i("router-link",{attrs:{to:"/update/item/"+t.itemid}},[i("small",[t._v("Edit..")])])],1)]),i("div",{staticClass:"item-detail"},[i("div",{domProps:{innerHTML:t._s(t.showDetail||"...")}}),i("el-button",{attrs:{type:"text",size:"mini"},on:{click:function(e){t.showShort=!t.showShort}}},[t._v("\n        "+t._s(t.showShort?"...More":"..Less")+"\n      ")])],1)],1),i("div",{staticClass:"include"},[i("rut-list",{attrs:{per:"item",id:t.itemid}})],1),i("div",{staticClass:"item-side"})]):t._e()},r=[],s=function(){var t=this,e=t.$createElement,i=t._self._c||e;return i("div",{staticClass:"item-sum"},[i("div",[i("img",{staticClass:"thumb",attrs:{src:t.cover,alt:"Cover",referrerPolicy:"no-referrer"}})]),i("div",{staticClass:"info"},[i("span",{staticClass:"title"},[t._v("\n      "+t._s(t.item.category)+" \n      "),t.out?[i("b",{staticStyle:{color:"#337ab7"}},[t._v(t._s(t.item.title))])]:[i("router-link",{attrs:{to:"/item/"+t.item.id}},[t._v(t._s(t.item.title))])]],2),i("table",{staticStyle:{"border-spacing":"0px"}},[i("tr",[t._m(0),i("td",[t._v(t._s(t.item.authors))])]),i("tr",[t._m(1),i("td",[t._v(t._s(t.item.publisher)+"   "+t._s(t.item.pub_at))])]),i("tr",[t._m(2),i("td",[t._v(t._s(t.item.uiid)+"   "+t._s(t.item.edition))])]),i("tr",[t._m(3),i("td",[t._v("\n          "+t._s(t.item.rut_count)+" \n        ")])])])]),i("div",{staticClass:"operate"},[i("el-dropdown",[i("el-button",{attrs:{type:"primary",size:"mini",plain:""}},[t._v("\n        "+t._s(t.flagAction)),i("i",{staticClass:"el-icon-arrow-down el-icon--right"})]),i("el-dropdown-menu",{attrs:{slot:"dropdown"},slot:"dropdown"},[i("el-dropdown-item",[i("el-button",{staticStyle:{color:"orange"},attrs:{type:"text"},on:{click:t.showAndloadRuts}},[t._v("\n                     Add to List\n          ")])],1),i("el-dropdown-item",[i("span",{on:{click:function(e){return t.toStar("todo")}}},[t._v("Todo")])]),i("el-dropdown-item",[i("span",{on:{click:function(e){return t.toStar("doing")}}},[t._v("Doing")])]),i("el-dropdown-item",[i("span",{on:{click:function(e){return t.toStar("done")}}},[t._v("Done")])])],1)],1)],1),i("el-dialog",{attrs:{title:"Add Item to one of Lists",width:"520px",visible:t.showAddtoRut},on:{"update:visible":function(e){t.showAddtoRut=e}}},[i("v-form",{ref:"form",staticClass:"add-form"},[i("el-select",{staticStyle:{width:"100%"},attrs:{filterable:"",remote:"","remote-method":t.storeKey,loading:t.searching,placeholder:"Search and Select a List"},nativeOn:{keyup:function(e){return!e.type.indexOf("key")&&t._k(e.keyCode,"enter",13,e.key,"Enter")?null:t.searchCreatedRuts(e)}},model:{value:t.rutID,callback:function(e){t.rutID=e},expression:"rutID"}},t._l(t.createdRuts,function(t){return i("el-option",{key:t.id,attrs:{label:t.title,value:t.id}})}),1),i("v-textarea",{attrs:{label:"Content",counter:"",rule:t.mustRule,rows:5,"auto-grow":""},model:{value:t.content,callback:function(e){t.content=e},expression:"content"}})],1),i("div",{staticClass:"dialog-footer",attrs:{slot:"footer"},slot:"footer"},[i("el-button",{attrs:{size:"mini",type:"success"},on:{click:t.addToRut}},[t._v("\n                 Add\n      ")])],1)],1),i("el-dialog",{attrs:{width:"450px",visible:t.showStar},on:{"update:visible":function(e){t.showStar=e}}},[i("v-form",{ref:"form",staticClass:"note-form"},[i("v-text-field",{attrs:{label:"Some Note: Optional, Max 42 words",counter:42,rules:t.lenRule},model:{value:t.note,callback:function(e){t.note=e},expression:"note"}})],1),i("div",{staticClass:"dialog-footer",attrs:{slot:"footer"},slot:"footer"},[i("el-button",{attrs:{size:"mini",type:"success"},on:{click:t.starAndNote}},[t._v("\n                 "+t._s("As "+t.starTo)+"\n      ")])],1)],1)],1)},o=[function(){var t=this,e=t.$createElement,i=t._self._c||e;return i("td",[i("small",{staticClass:"indicator"},[t._v("Byline")])])},function(){var t=this,e=t.$createElement,i=t._self._c||e;return i("td",[i("small",{staticClass:"indicator"},[t._v("Publish ")])])},function(){var t=this,e=t.$createElement,i=t._self._c||e;return i("td",[i("small",{staticClass:"indicator"},[t._v("Edition")])])},function(){var t=this,e=t.$createElement,i=t._self._c||e;return i("td",[i("small",{staticClass:"indicator"},[t._v("Listed")])])}],a=i("d722"),c=i("0a5a"),u={name:"item-sum",props:["item","out"],components:{},data:function(){return{showAddtoRut:!1,createdRuts:[],searchKey:"",searching:!1,rutID:"",content:"",flagAction:"Options",flagNote:"",flagTime:"",note:"",showStar:!1,starTo:"",lenRule:[function(t){return t.length<=42||"Must be less than 42"}],mustRule:[function(t){return!!t||"required"}]}},computed:{cover:function(){return this.item.cover}},methods:{checkStar:function(){var t=this;if(Object(c["a"])()){var e=this.item.id||this.$route.params.id;Object(a["c"])(e).then(function(e){t.flagAction=e.data.message,t.flagNote=e.data.note,t.flagTime=e.data.when})}else this.flagAction="Options",this.flagNote=""},toStar:function(t){Object(c["a"])()?(this.showStar=!0,this.starTo=t):(this.$message("Should Log in to Access"),this.$router.push({path:"/login",query:{redirect:this.$route.fullPath}}))},starAndNote:function(){var t=this;if(this.$refs.form.validate()&&Object(c["a"])()){var e=this.note.trim(),i=this.starTo;Object(a["w"])(this.item.id,i,e||i).then(function(e){t.flagAction=e.data.message,t.flagNote=e.data.note}),this.showStar=!1}else this.$message("Invalid Input or Need to Log in")},storeKey:function(t){""!==t.trim()&&(this.searchKey=t.trim())},searchCreatedRuts:function(){var t=this;if(Object(c["a"])()){this.searching=!0;var e=this.$store.getters.actID;this.searchKey.length<6?Object(a["n"])("user",e,1,"create").then(function(e){t.createdRuts=e.data.ruts,t.searching=!1}):this.$http("".concat(a["a"],"/ruts/key/").concat(e,"?keyword=").concat(this.searchKey,"&from=user")).then(function(e){t.createdRuts=e.data.ruts,t.searching=!1})}},showAndloadRuts:function(){Object(c["a"])()&&(this.searchCreatedRuts(),this.showAddtoRut=!0)},addToRut:function(){var t=this;if(this.rutID&&Object(c["a"])()){var e={rut_id:this.rutID,item_id:this.item.id,item_order:0,content:this.content,uname:""};Object(a["f"])(this.rutID,e).then(function(){var e={rutid:t.rutID,lastUpdate:0,ref:"lastUpdate"};t.$store.commit("RENEW_RUT",e),t.showAddtoRut=!1,t.$router.push("/r/".concat(t.rutID))})}else this.$message("Invalid Input or Need to Log in")}},created:function(){this.checkStar()}},l=u,d=(i("3661"),i("2877")),f=Object(d["a"])(l,s,o,!1,null,"6c1cb60f",null),h=f.exports,m=i("0d83"),p=i("e6d6"),v=i("5ad4"),_={name:"item-view",title:function(){return this.itemTitle},components:{ItemSum:h,RutList:m["a"]},data:function(){return{itemid:"",itemTitle:"",itemDetail:"",showShort:!0}},computed:{item:function(){return this.$store.state.item.items[this.$route.params.id]},showDetail:function(){var t=Object(p["a"])(this.itemDetail),e=255,i=t.length>e&&this.showShort;return i?Object(v["showLess"])(t,e):(this.showShort=!1,t)}},methods:{loadItem:function(){var t=this,e=this.$route.params.id;this.$store.dispatch("getItem",e).then(function(e){t.itemDetail=e.detail,t.itemTitle=e.title,t.itemid=e.id})}},created:function(){this.loadItem()}},b=_,g=(i("c757"),Object(d["a"])(b,n,r,!1,null,"342261ae",null));e["default"]=g.exports},3661:function(t,e,i){"use strict";var n=i("4ca7"),r=i.n(n);r.a},"4ca7":function(t,e,i){},"549b":function(t,e,i){"use strict";var n=i("d864"),r=i("63b6"),s=i("241e"),o=i("b0dc"),a=i("3702"),c=i("b447"),u=i("20fd"),l=i("7cd6");r(r.S+r.F*!i("4ee1")(function(t){Array.from(t)}),"Array",{from:function(t){var e,i,r,d,f=s(t),h="function"==typeof this?this:Array,m=arguments.length,p=m>1?arguments[1]:void 0,v=void 0!==p,_=0,b=l(f);if(v&&(p=n(p,m>2?arguments[2]:void 0,2)),void 0==b||h==Array&&a(b))for(e=c(f.length),i=new h(e);e>_;_++)u(i,_,v?p(f[_],_):f[_]);else for(d=b.call(f),i=new h;!(r=d.next()).done;_++)u(i,_,v?o(d,p,[r.value,_],!0):r.value);return i.length=_,i}})},"54a1":function(t,e,i){i("6c1c"),i("1654"),t.exports=i("95d5")},"75fc":function(t,e,i){"use strict";var n=i("a745"),r=i.n(n);function s(t){if(r()(t)){for(var e=0,i=new Array(t.length);e<t.length;e++)i[e]=t[e];return i}}var o=i("774e"),a=i.n(o),c=i("c8bb"),u=i.n(c);function l(t){if(u()(Object(t))||"[object Arguments]"===Object.prototype.toString.call(t))return a()(t)}function d(){throw new TypeError("Invalid attempt to spread non-iterable instance")}function f(t){return s(t)||l(t)||d()}i.d(e,"a",function(){return f})},"774e":function(t,e,i){t.exports=i("d2d5")},"95d5":function(t,e,i){var n=i("40c3"),r=i("5168")("iterator"),s=i("481b");t.exports=i("584a").isIterable=function(t){var e=Object(t);return void 0!==e[r]||"@@iterator"in e||s.hasOwnProperty(n(e))}},a05d:function(t,e,i){"use strict";var n=i("bf1d"),r=i.n(n);r.a},a745:function(t,e,i){t.exports=i("f410")},bf1d:function(t,e,i){},bf69:function(t,e,i){},c757:function(t,e,i){"use strict";var n=i("eac3"),r=i.n(n);r.a},c8bb:function(t,e,i){t.exports=i("54a1")},d2d5:function(t,e,i){i("1654"),i("549b"),t.exports=i("584a").Array.from},eac3:function(t,e,i){},f410:function(t,e,i){i("1af6"),t.exports=i("584a").Array.isArray}}]);
//# sourceMappingURL=chunk-bb6391f2.c7d88b62.js.map