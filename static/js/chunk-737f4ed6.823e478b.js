(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["chunk-737f4ed6"],{"04c6":function(t,e,i){"use strict";i.d(e,"b",function(){return l}),i.d(e,"a",function(){return a});var l=/^(https?):\/\/([^\/:]+)(:[0-9]+)?(\/.*)?$/,a=/[-_ ]/gi},"3a6b":function(t,e,i){},"45a1":function(t,e,i){"use strict";var l=i("3a6b"),a=i.n(l);a.a},"635f":function(t,e,i){"use strict";i.r(e);var l=function(){var t=this,e=t.$createElement,i=t._self._c||e;return i("div",{staticClass:"submit-page"},[t._m(0),i("v-form",{ref:"form",staticClass:"submit-form"},[i("v-textarea",{attrs:{label:"Title",counter:144,rules:t.titleRule,rows:1,"auto-grow":""},model:{value:t.title,callback:function(e){t.title=e},expression:"title"}}),i("v-text-field",{attrs:{label:"UID: ISBN.."},model:{value:t.uiid,callback:function(e){t.uiid=e},expression:"uiid"}}),i("v-text-field",{attrs:{label:"Author or Instructor",counter:144,rules:t.titleRule},model:{value:t.authors,callback:function(e){t.authors=e},expression:"authors"}}),i("v-textarea",{attrs:{label:"Cover Image Url",counter:144,rules:t.lenRule,rows:1,"auto-grow":""},model:{value:t.cover,callback:function(e){t.cover=e},expression:"cover"}}),i("v-textarea",{attrs:{label:"Resource URL",counter:144,rules:t.lenRule,rows:1,"auto-grow":""},model:{value:t.url,callback:function(e){t.url=e},expression:"url"}}),i("v-text-field",{attrs:{label:"Edition",counter:144,rules:t.lenRule},model:{value:t.edition,callback:function(e){t.edition=e},expression:"edition"}}),i("v-text-field",{attrs:{label:"Publish Date"},model:{value:t.pubDate,callback:function(e){t.pubDate=e},expression:"pubDate"}}),i("v-text-field",{attrs:{label:"Publisher",counter:144,rules:t.lenRule},model:{value:t.publisher,callback:function(e){t.publisher=e},expression:"publisher"}}),i("v-text-field",{attrs:{label:"Category"},model:{value:t.category,callback:function(e){t.category=e},expression:"category"}}),i("v-textarea",{attrs:{label:"Detail",counter:"","auto-grow":""},model:{value:t.detail,callback:function(e){t.detail=e},expression:"detail"}})],1),i("el-button",{staticClass:"blockbtn",attrs:{type:"primary",size:"small"},on:{click:t.onSubmit}},[t._v("Submit\n  ")])],1)},a=[function(){var t=this,e=t.$createElement,i=t._self._c||e;return i("div",{staticClass:"title"},[i("span",[t._v("Submit A New Item ")]),i("small",{staticStyle:{color:"green","font-size":"0.8em"}},[t._v("books, courses, etc.")])])}],r=(i("a481"),i("d722")),u=i("0a5a"),o=i("04c6"),s={name:"new-item",title:"Submit New Item",components:{},data:function(){return{title:"",uiid:"",authors:"",pubDate:"",publisher:"",category:"Book",url:"",cover:"",edition:"",detail:"",mustRule:[function(t){return!!t||"required"}],lenRule:[function(t){return t.length<=144||"Must be less than 144 characters"}]}},computed:{titleRule:function(){return this.mustRule.concat(this.lenRule)}},methods:{onSubmit:function(){var t=this;if(this.$refs.form.validate()&&Object(u["a"])()){var e=this.uiid.replace(o["a"],""),i={title:this.title.trim(),uiid:e,authors:this.authors.trim(),pub_at:this.pubDate.trim(),publisher:this.publisher.trim(),category:this.category.trim(),url:this.url.trim(),cover:this.cover.trim(),edition:this.edition.trim(),detail:this.detail.trim()};Object(r["s"])(i).then(function(e){var i=e.data.item.id;t.$router.push("/item/".concat(i))})}else this.$message("Invalid Input or Need to Log in")}}},n=s,c=(i("45a1"),i("2877")),b=Object(c["a"])(n,l,a,!1,null,"c2ca46a8",null);e["default"]=b.exports}}]);
//# sourceMappingURL=chunk-737f4ed6.823e478b.js.map