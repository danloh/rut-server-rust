(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["chunk-7e287dee"],{"04c6":function(e,t,s){"use strict";s.d(t,"a",function(){return a});var a=/^(https?):\/\/([^\/:]+)(:[0-9]+)?(\/.*)?$/},"5a00":function(e,t,s){"use strict";s.r(t);var a=function(){var e=this,t=e.$createElement,s=e._self._c||t;return s("div",{staticClass:"sign-page"},[s("h3",{staticClass:"title"},[e._v("Welcome to Join")]),s("v-form",{ref:"form",staticClass:"sign-form"},[s("v-text-field",{attrs:{label:"Username",counter:16,rules:e.nameRule},model:{value:e.uname,callback:function(t){e.uname=t},expression:"uname"}}),s("v-text-field",{attrs:{label:"Password",type:"password",rules:e.pswRule},model:{value:e.password,callback:function(t){e.password=t},expression:"password"}}),s("v-text-field",{attrs:{label:"Confirm Password",type:"password",rules:e.repswRule},model:{value:e.repassword,callback:function(t){e.repassword=t},expression:"repassword"}}),s("el-button",{staticClass:"blockbtn",attrs:{type:"primary",size:"small"},on:{click:e.onReg}},[e._v("Register\n    ")])],1),s("router-link",{attrs:{to:"/login"}},[e._v("Have an Account? Login")])],1)},r=[],n=s("d722"),o=(s("04c6"),{name:"register",title:"Register",data:function(){var e=this;return{uname:"",nameRule:[function(e){return!!e||"required"},function(e){return e.length<=16||"Must be less than 16 characters"}],password:"",pswRule:[function(e){return e.length>=8||"Must be more than 8 characters"}],repassword:"",repswRule:[function(t){return(!!t&&t)===e.password||"Not Match"}]}},methods:{onReg:function(){var e=this;if(this.$refs.form.validate()){var t={uname:this.uname,password:this.password,confirm_password:this.repassword};Object(n["v"])(t).then(function(t){200==t.data.status?e.$router.push("/login"):409==t.data.status?alert("duplicated user"):e.$router.push("/register")})}else console.log("Invalid")}}}),u=o,i=(s("dad9"),s("2877")),l=Object(i["a"])(u,a,r,!1,null,"2dd9ea86",null);t["default"]=l.exports},b2cc:function(e,t,s){},dad9:function(e,t,s){"use strict";var a=s("b2cc"),r=s.n(a);r.a}}]);
//# sourceMappingURL=chunk-7e287dee.c947fcb9.js.map