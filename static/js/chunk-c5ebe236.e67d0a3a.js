(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["chunk-c5ebe236"],{"04c6":function(t,e,a){"use strict";a.d(e,"b",function(){return s}),a.d(e,"a",function(){return n});var s=/^(https?):\/\/([^\/:]+)(:[0-9]+)?(\/.*)?$/,n=/[-_ ]/gi},"2b47":function(t,e,a){"use strict";a.r(e);var s=function(){var t=this,e=t.$createElement,a=t._self._c||e;return a("div",[a("el-button",{attrs:{size:"mini"},on:{click:function(e){t.show=!t.show}}},[t._v("\n    "+t._s(t.show?"Change Password":"Update Profile")+"\n  ")]),t.show?a("div",{staticClass:"update-page"},[a("h3",{staticClass:"title"},[t._v("Update My Profile")]),a("v-form",{ref:"form",staticClass:"update-form"},[a("v-text-field",{attrs:{label:"Nickname",counter:16,rules:t.nameRule},model:{value:t.nickname,callback:function(e){t.nickname=e},expression:"nickname"}}),a("v-textarea",{attrs:{label:"Avatar",counter:144,rules:t.lenRule,rows:1,"auto-grow":""},model:{value:t.avatar,callback:function(e){t.avatar=e},expression:"avatar"}}),a("v-text-field",{attrs:{label:"Location",rules:t.lenRule},model:{value:t.location,callback:function(e){t.location=e},expression:"location"}}),a("v-textarea",{attrs:{label:"Intro",counter:"",rows:6,"auto-grow":""},model:{value:t.intro,callback:function(e){t.intro=e},expression:"intro"}})],1),a("el-button",{staticClass:"blockbtn",attrs:{type:"primary",size:"small"},on:{click:t.onUpdate}},[t._v("Update My Profile\n    ")])],1):t._e(),t.show?t._e():a("div",{staticClass:"update-page"},[a("h3",{staticClass:"title"},[t._v("Change My Password")]),a("v-form",{ref:"form",staticClass:"psw-form"},[a("v-text-field",{attrs:{label:"Old Password",type:"password",rules:t.pswRule},model:{value:t.oldpsw,callback:function(e){t.oldpsw=e},expression:"oldpsw"}}),a("v-text-field",{attrs:{label:"New Password",type:"password",rules:t.pswRule},model:{value:t.psw,callback:function(e){t.psw=e},expression:"psw"}}),a("v-text-field",{attrs:{label:"Confirm Password",type:"password",rules:t.repswRule},model:{value:t.repsw,callback:function(e){t.repsw=e},expression:"repsw"}})],1),a("el-button",{staticClass:"blockbtn",attrs:{type:"primary",size:"small"},on:{click:t.onChangePsw}},[t._v("Change Password\n    ")])],1)],1)},n=[],r=a("d722"),i=(a("04c6"),a("0a5a")),o={name:"update-profile",title:"Update My Profile",data:function(){var t=this;return{uname:"",avatar:"",email:"",location:"",intro:"",nickname:"",nameRule:[function(t){return t.length<=16||"Must be less than 16 characters"}],lenRule:[function(t){return t.length<=144||"Must be less than 144 characters"}],emailRule:[function(t){return/.+@.+/.test(t)||"E-mail must be valid"}],show:!0,oldpsw:"",psw:"",pswRule:[function(t){return!!t||"required"},function(t){return t.length>=8||"Must be more than 8 characters"}],repsw:"",repswRule:[function(e){return(!!e&&e)===t.psw||"Not Match"}]}},methods:{onUpdate:function(){var t=this,e=this.$store.getters.actID;if(this.$refs.form.validate()&&Object(i["a"])()&&this.uname===e){var a={uname:this.uname.trim(),avatar:this.avatar.trim(),email:this.email.trim(),intro:this.intro.trim(),location:this.location.trim(),nickname:this.nickname.trim()||this.uname};Object(r["E"])(this.uname,a).then(function(e){var a=e.data.user.uname;t.$router.push("/p/".concat(a))})}else this.$message("Invalid Input or Need to Log in")},loadUser:function(){var t=this,e=this.$route.params.id;Object(r["q"])(e).then(function(e){var a=e.data.user;t.uname=a.uname,t.avatar=a.avatar,t.email=a.email,t.intro=a.intro,t.location=a.location,t.nickname=a.nickname})},onChangePsw:function(){var t=this,e=this.$store.getters.actID;if(this.$refs.form.validate()&&Object(i["a"])()&&this.uname===e){var a={old_psw:this.oldpsw.trim(),new_psw:this.psw.trim(),uname:this.uname};Object(r["b"])(this.uname,a).then(function(e){200==e.data.status?t.$router.push("/login"):t.$message(e.data.message)})}else console.log("Invalid")}},created:function(){this.loadUser()}},l=o,c=(a("ec9e"),a("2877")),u=Object(c["a"])(l,s,n,!1,null,"dba58704",null);e["default"]=u.exports},"7a0c":function(t,e,a){},ec9e:function(t,e,a){"use strict";var s=a("7a0c"),n=a.n(s);n.a}}]);
//# sourceMappingURL=chunk-c5ebe236.e67d0a3a.js.map