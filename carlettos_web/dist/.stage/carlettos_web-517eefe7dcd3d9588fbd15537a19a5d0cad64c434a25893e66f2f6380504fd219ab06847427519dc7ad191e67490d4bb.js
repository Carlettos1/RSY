let Q=0,P=null,S=`undefined`,Y=`boolean`,N=128,Z=`string`,W=1,_=`Object`,T=`utf-8`,X=`number`,a0=4,V=`function`,M=Array,U=Error,$=JSON.stringify,a1=Object,R=Uint8Array,O=undefined;var G=(async(a,b)=>{if(typeof Response===V&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===V){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var u=(a=>{const b=typeof a;if(b==X||b==Y||a==P){return `${a}`};if(b==Z){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==P){return `Symbol`}else{return `Symbol(${b})`}};if(b==V){const b=a.name;if(typeof b==Z&&b.length>Q){return `Function(${b})`}else{return `Function`}};if(M.isArray(a)){const b=a.length;let c=`[`;if(b>Q){c+=u(a[Q])};for(let d=W;d<b;d++){c+=`, `+ u(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>W){d=c[W]}else{return toString.call(a)};if(d==_){try{return `Object(`+ $(a)+ `)`}catch(a){return _}};if(a instanceof U){return `${a.name}: ${a.message}\n${a.stack}`};return d});var y=((c,d,e)=>{try{a._dyn_core__ops__function__Fn___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h5e59fc2f55d740e7(c,d,x(e))}finally{b[w++]=O}});var I=((a,b)=>{});var F=((a,b)=>{a=a>>>Q;const c=E();const d=c.subarray(a/a0,a/a0+ b);const e=[];for(let a=Q;a<d.length;a++){e.push(f(d[a]))};return e});var f=(a=>{const b=c(a);e(a);return b});var o=(a=>{if(d===b.length)b.push(b.length+ W);const c=d;d=b[c];b[c]=a;return c});var L=(async(b)=>{if(a!==O)return a;if(typeof b===S){b=new URL(`carlettos_web-517eefe7dcd3d9588fbd15537a19a5d0cad64c434a25893e66f2f6380504fd219ab06847427519dc7ad191e67490d4bb_bg.wasm`,import.meta.url)};const c=H();if(typeof b===Z||typeof Request===V&&b instanceof Request||typeof URL===V&&b instanceof URL){b=fetch(b)};I(c);const {instance:d,module:e}=await G(await b,c);return J(d,e)});var K=(b=>{if(a!==O)return a;const c=H();I(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return J(d,b)});function C(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(o(b))}}var J=((b,c)=>{a=b.exports;L.__wbindgen_wasm_module=c;s=P;m=P;D=P;h=P;a.__wbindgen_start();return a});var H=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_drop_ref=(a=>{f(a)});b.wbg.__wbindgen_cb_drop=(a=>{const b=f(a).original;if(b.cnt--==W){b.a=Q;return !0};const c=!1;return c});b.wbg.__wbindgen_json_serialize=((b,d)=>{const e=c(d);const f=$(e===O?P:e);const h=l(f,a.__wbindgen_malloc,a.__wbindgen_realloc);const i=g;n()[b/a0+ W]=i;n()[b/a0+ Q]=h});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return o(b)});b.wbg.__wbindgen_string_get=((b,d)=>{const e=c(d);const f=typeof e===Z?e:O;var h=p(f)?Q:l(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var i=g;n()[b/a0+ W]=i;n()[b/a0+ Q]=h});b.wbg.__wbindgen_is_string=(a=>{const b=typeof c(a)===Z;return b});b.wbg.__wbindgen_number_new=(a=>{const b=a;return o(b)});b.wbg.__wbindgen_string_new=((a,b)=>{const c=r(a,b);return o(c)});b.wbg.__wbg_listenerid_6dcf1c62b7b7de58=((a,b)=>{const d=c(b).__yew_listener_id;n()[a/a0+ W]=p(d)?Q:d;n()[a/a0+ Q]=!p(d)});b.wbg.__wbg_setlistenerid_f2e783343fa0cec1=((a,b)=>{c(a).__yew_listener_id=b>>>Q});b.wbg.__wbg_cachekey_b81c1aacc6a0645c=((a,b)=>{const d=c(b).__yew_subtree_cache_key;n()[a/a0+ W]=p(d)?Q:d;n()[a/a0+ Q]=!p(d)});b.wbg.__wbg_subtreeid_e80a1798fee782f9=((a,b)=>{const d=c(b).__yew_subtree_id;n()[a/a0+ W]=p(d)?Q:d;n()[a/a0+ Q]=!p(d)});b.wbg.__wbg_setsubtreeid_e1fab6b578c800cf=((a,b)=>{c(a).__yew_subtree_id=b>>>Q});b.wbg.__wbg_setcachekey_75bcd45312087529=((a,b)=>{c(a).__yew_subtree_cache_key=b>>>Q});b.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new U();return o(a)});b.wbg.__wbg_stack_658279fe44541cf6=((b,d)=>{const e=c(d).stack;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a0+ W]=h;n()[b/a0+ Q]=f});b.wbg.__wbg_error_f851667af71bcfc6=((b,c)=>{let d;let e;try{d=b;e=c;console.error(r(b,c))}finally{a.__wbindgen_free(d,e,W)}});b.wbg.__wbg_queueMicrotask_4d890031a6a5a50c=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_adae4bc085237231=(a=>{const b=c(a).queueMicrotask;return o(b)});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===V;return b});b.wbg.__wbindgen_is_object=(a=>{const b=c(a);const d=typeof b===`object`&&b!==P;return d});b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===O;return b});b.wbg.__wbindgen_in=((a,b)=>{const d=c(a) in c(b);return d});b.wbg.__wbindgen_error_new=((a,b)=>{const c=new U(r(a,b));return o(c)});b.wbg.__wbg_crypto_58f13aa23ffcb166=(a=>{const b=c(a).crypto;return o(b)});b.wbg.__wbg_process_5b786e71d465a513=(a=>{const b=c(a).process;return o(b)});b.wbg.__wbg_versions_c2ab80650590b6a2=(a=>{const b=c(a).versions;return o(b)});b.wbg.__wbg_node_523d7bd03ef69fba=(a=>{const b=c(a).node;return o(b)});b.wbg.__wbg_msCrypto_abcb1295e768d1f2=(a=>{const b=c(a).msCrypto;return o(b)});b.wbg.__wbg_require_2784e593a4674877=function(){return C((()=>{const a=module.require;return o(a)}),arguments)};b.wbg.__wbg_randomFillSync_a0d98aa11c81fe89=function(){return C(((a,b)=>{c(a).randomFillSync(f(b))}),arguments)};b.wbg.__wbg_getRandomValues_504510b5564925af=function(){return C(((a,b)=>{c(a).getRandomValues(c(b))}),arguments)};b.wbg.__wbindgen_jsval_loose_eq=((a,b)=>{const d=c(a)==c(b);return d});b.wbg.__wbindgen_boolean_get=(a=>{const b=c(a);const d=typeof b===Y?(b?W:Q):2;return d});b.wbg.__wbindgen_number_get=((a,b)=>{const d=c(b);const e=typeof d===X?d:O;t()[a/8+ W]=p(e)?Q:e;n()[a/a0+ Q]=!p(e)});b.wbg.__wbindgen_as_number=(a=>{const b=+c(a);return b});b.wbg.__wbg_getwithrefkey_4a92a5eca60879b9=((a,b)=>{const d=c(a)[c(b)];return o(d)});b.wbg.__wbg_set_9182712abebf82ef=((a,b,d)=>{c(a)[f(b)]=f(d)});b.wbg.__wbg_error_a526fb08a0205972=((b,c)=>{var d=F(b,c).slice();a.__wbindgen_free(b,c*a0,a0);console.error(...d)});b.wbg.__wbg_body_64abc9aba1891e91=(a=>{const b=c(a).body;return p(b)?Q:o(b)});b.wbg.__wbg_createElement_fdd5c113cb84539e=function(){return C(((a,b,d)=>{const e=c(a).createElement(r(b,d));return o(e)}),arguments)};b.wbg.__wbg_createElementNS_524b05a6070757b6=function(){return C(((a,b,d,e,f)=>{const g=c(a).createElementNS(b===Q?O:r(b,d),r(e,f));return o(g)}),arguments)};b.wbg.__wbg_createTextNode_7ff0c034b2855f66=((a,b,d)=>{const e=c(a).createTextNode(r(b,d));return o(e)});b.wbg.__wbg_querySelector_c72dce5ac4b6bc3e=function(){return C(((a,b,d)=>{const e=c(a).querySelector(r(b,d));return p(e)?Q:o(e)}),arguments)};b.wbg.__wbg_instanceof_Window_3e5cd1f48c152d01=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_document_d609202d16c38224=(a=>{const b=c(a).document;return p(b)?Q:o(b)});b.wbg.__wbg_location_176c34e89c2c9d80=(a=>{const b=c(a).location;return o(b)});b.wbg.__wbg_history_80998b7456bf367e=function(){return C((a=>{const b=c(a).history;return o(b)}),arguments)};b.wbg.__wbg_fetch_6c415b3a07763878=((a,b)=>{const d=c(a).fetch(c(b));return o(d)});b.wbg.__wbg_instanceof_Element_3f326a19cc457941=(a=>{let b;try{b=c(a) instanceof Element}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_namespaceURI_7cc7ef157e398356=((b,d)=>{const e=c(d).namespaceURI;var f=p(e)?Q:l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=g;n()[b/a0+ W]=h;n()[b/a0+ Q]=f});b.wbg.__wbg_setinnerHTML_ce0d6527ce4086f2=((a,b,d)=>{c(a).innerHTML=r(b,d)});b.wbg.__wbg_outerHTML_b5a8d952b5615778=((b,d)=>{const e=c(d).outerHTML;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a0+ W]=h;n()[b/a0+ Q]=f});b.wbg.__wbg_removeAttribute_2e200daefb9f3ed4=function(){return C(((a,b,d)=>{c(a).removeAttribute(r(b,d))}),arguments)};b.wbg.__wbg_setAttribute_e7b72a5e7cfcb5a3=function(){return C(((a,b,d,e,f)=>{c(a).setAttribute(r(b,d),r(e,f))}),arguments)};b.wbg.__wbg_addEventListener_374cbfd2bbc19ccf=function(){return C(((a,b,d,e,f)=>{c(a).addEventListener(r(b,d),c(e),c(f))}),arguments)};b.wbg.__wbg_removeEventListener_9ece7e86d1135657=function(){return C(((a,b,d,e,f)=>{c(a).removeEventListener(r(b,d),c(e),f!==Q)}),arguments)};b.wbg.__wbg_href_160af2ae1328d7b7=function(){return C(((b,d)=>{const e=c(d).href;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a0+ W]=h;n()[b/a0+ Q]=f}),arguments)};b.wbg.__wbg_pathname_1ab7e82aaa4511ff=function(){return C(((b,d)=>{const e=c(d).pathname;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a0+ W]=h;n()[b/a0+ Q]=f}),arguments)};b.wbg.__wbg_search_9f7ca8896c2d0804=function(){return C(((b,d)=>{const e=c(d).search;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a0+ W]=h;n()[b/a0+ Q]=f}),arguments)};b.wbg.__wbg_hash_be2940ca236b5efc=function(){return C(((b,d)=>{const e=c(d).hash;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a0+ W]=h;n()[b/a0+ Q]=f}),arguments)};b.wbg.__wbg_newwithstrandinit_f581dff0d19a8b03=function(){return C(((a,b,d)=>{const e=new Request(r(a,b),c(d));return o(e)}),arguments)};b.wbg.__wbg_parentNode_92a7017b3a4fad43=(a=>{const b=c(a).parentNode;return p(b)?Q:o(b)});b.wbg.__wbg_parentElement_72e144c2e8d9e0b5=(a=>{const b=c(a).parentElement;return p(b)?Q:o(b)});b.wbg.__wbg_childNodes_a5762b4b3e073cf6=(a=>{const b=c(a).childNodes;return o(b)});b.wbg.__wbg_lastChild_a62e3fbaab87f734=(a=>{const b=c(a).lastChild;return p(b)?Q:o(b)});b.wbg.__wbg_nextSibling_bafccd3347d24543=(a=>{const b=c(a).nextSibling;return p(b)?Q:o(b)});b.wbg.__wbg_setnodeValue_630c6470d05b600e=((a,b,d)=>{c(a).nodeValue=b===Q?O:r(b,d)});b.wbg.__wbg_textContent_2f37235e13f8484b=((b,d)=>{const e=c(d).textContent;var f=p(e)?Q:l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=g;n()[b/a0+ W]=h;n()[b/a0+ Q]=f});b.wbg.__wbg_cloneNode_405d5ea3f7e0098a=function(){return C((a=>{const b=c(a).cloneNode();return o(b)}),arguments)};b.wbg.__wbg_insertBefore_726c1640c419e940=function(){return C(((a,b,d)=>{const e=c(a).insertBefore(c(b),c(d));return o(e)}),arguments)};b.wbg.__wbg_removeChild_942eb9c02243d84d=function(){return C(((a,b)=>{const d=c(a).removeChild(c(b));return o(d)}),arguments)};b.wbg.__wbg_new_7a20246daa6eec7e=function(){return C((()=>{const a=new Headers();return o(a)}),arguments)};b.wbg.__wbg_href_f21dc804d4da134a=((b,d)=>{const e=c(d).href;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a0+ W]=h;n()[b/a0+ Q]=f});b.wbg.__wbg_value_57e57170f6952449=((b,d)=>{const e=c(d).value;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a0+ W]=h;n()[b/a0+ Q]=f});b.wbg.__wbg_setvalue_a11f3069fd7a1805=((a,b,d)=>{c(a).value=r(b,d)});b.wbg.__wbg_bubbles_f1cdd0584446cad0=(a=>{const b=c(a).bubbles;return b});b.wbg.__wbg_cancelBubble_976cfdf7ac449a6c=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_composedPath_12a068e57a98cf90=(a=>{const b=c(a).composedPath();return o(b)});b.wbg.__wbg_preventDefault_7f821f72e7c6b5d4=(a=>{c(a).preventDefault()});b.wbg.__wbg_href_e9aac3826080dcaa=((b,d)=>{const e=c(d).href;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a0+ W]=h;n()[b/a0+ Q]=f});b.wbg.__wbg_pathname_aeafa820be91c325=((b,d)=>{const e=c(d).pathname;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a0+ W]=h;n()[b/a0+ Q]=f});b.wbg.__wbg_search_f6e95882a48d3f69=((b,d)=>{const e=c(d).search;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a0+ W]=h;n()[b/a0+ Q]=f});b.wbg.__wbg_setsearch_4f7d084e0d811add=((a,b,d)=>{c(a).search=r(b,d)});b.wbg.__wbg_hash_0087751acddc8f2a=((b,d)=>{const e=c(d).hash;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a0+ W]=h;n()[b/a0+ Q]=f});b.wbg.__wbg_sethash_bfc9db317a77305c=((a,b,d)=>{c(a).hash=r(b,d)});b.wbg.__wbg_new_9e08fd37c1c53142=function(){return C(((a,b)=>{const c=new URL(r(a,b));return o(c)}),arguments)};b.wbg.__wbg_newwithbase_f4989aa5bbd5cc29=function(){return C(((a,b,c,d)=>{const e=new URL(r(a,b),r(c,d));return o(e)}),arguments)};b.wbg.__wbg_ctrlKey_643b17aaac67db50=(a=>{const b=c(a).ctrlKey;return b});b.wbg.__wbg_shiftKey_8fb7301f56e7e01c=(a=>{const b=c(a).shiftKey;return b});b.wbg.__wbg_altKey_c6c2a7e797d9a669=(a=>{const b=c(a).altKey;return b});b.wbg.__wbg_metaKey_2a8dbd51a3f59e9c=(a=>{const b=c(a).metaKey;return b});b.wbg.__wbg_state_ba77b2c3ee29c912=function(){return C((a=>{const b=c(a).state;return o(b)}),arguments)};b.wbg.__wbg_pushState_e159043fce8f87bc=function(){return C(((a,b,d,e,f,g)=>{c(a).pushState(c(b),r(d,e),f===Q?O:r(f,g))}),arguments)};b.wbg.__wbg_instanceof_Response_4c3b1446206114d1=(a=>{let b;try{b=c(a) instanceof Response}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_json_34535d9848f043eb=function(){return C((a=>{const b=c(a).json();return o(b)}),arguments)};b.wbg.__wbg_instanceof_ShadowRoot_0bd39e89ab117f86=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_09eee5e3d9cf59a1=(a=>{const b=c(a).host;return o(b)});b.wbg.__wbg_setchecked_c1d5c3726082e274=((a,b)=>{c(a).checked=b!==Q});b.wbg.__wbg_value_e024243a9dae20bc=((b,d)=>{const e=c(d).value;const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a0+ W]=h;n()[b/a0+ Q]=f});b.wbg.__wbg_setvalue_5b3442ff620b4a5d=((a,b,d)=>{c(a).value=r(b,d)});b.wbg.__wbg_get_f01601b5a68d10e3=((a,b)=>{const d=c(a)[b>>>Q];return o(d)});b.wbg.__wbg_length_1009b1af0c481d7b=(a=>{const b=c(a).length;return b});b.wbg.__wbg_newnoargs_c62ea9419c21fbac=((a,b)=>{const c=new Function(r(a,b));return o(c)});b.wbg.__wbg_call_90c26b09837aba1c=function(){return C(((a,b)=>{const d=c(a).call(c(b));return o(d)}),arguments)};b.wbg.__wbg_new_9fb8d994e1c0aaac=(()=>{const a=new a1();return o(a)});b.wbg.__wbg_self_f0e34d89f33b99fd=function(){return C((()=>{const a=self.self;return o(a)}),arguments)};b.wbg.__wbg_window_d3b084224f4774d7=function(){return C((()=>{const a=window.window;return o(a)}),arguments)};b.wbg.__wbg_globalThis_9caa27ff917c6860=function(){return C((()=>{const a=globalThis.globalThis;return o(a)}),arguments)};b.wbg.__wbg_global_35dfdd59a4da3e74=function(){return C((()=>{const a=global.global;return o(a)}),arguments)};b.wbg.__wbg_from_71add2e723d1f1b2=(a=>{const b=M.from(c(a));return o(b)});b.wbg.__wbg_instanceof_ArrayBuffer_e7d53d51371448e2=(a=>{let b;try{b=c(a) instanceof ArrayBuffer}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_instanceof_Error_31ca8d97f188bfbc=(a=>{let b;try{b=c(a) instanceof U}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_message_55b9ea8030688597=(a=>{const b=c(a).message;return o(b)});b.wbg.__wbg_name_e5eede664187fed6=(a=>{const b=c(a).name;return o(b)});b.wbg.__wbg_toString_a44236e90224e279=(a=>{const b=c(a).toString();return o(b)});b.wbg.__wbg_call_5da1969d7cd31ccd=function(){return C(((a,b,d)=>{const e=c(a).call(c(b),c(d));return o(e)}),arguments)};b.wbg.__wbg_isSafeInteger_f93fde0dca9820f8=(a=>{const b=Number.isSafeInteger(c(a));return b});b.wbg.__wbg_entries_9e2e2aa45aa5094a=(a=>{const b=a1.entries(c(a));return o(b)});b.wbg.__wbg_is_ff7acd231c75c0e4=((a,b)=>{const d=a1.is(c(a),c(b));return d});b.wbg.__wbg_resolve_6e1c6553a82f85b7=(a=>{const b=Promise.resolve(c(a));return o(b)});b.wbg.__wbg_then_3ab08cd4fbb91ae9=((a,b)=>{const d=c(a).then(c(b));return o(d)});b.wbg.__wbg_then_8371cc12cfedc5a2=((a,b,d)=>{const e=c(a).then(c(b),c(d));return o(e)});b.wbg.__wbg_buffer_a448f833075b71ba=(a=>{const b=c(a).buffer;return o(b)});b.wbg.__wbg_newwithbyteoffsetandlength_d0482f893617af71=((a,b,d)=>{const e=new R(c(a),b>>>Q,d>>>Q);return o(e)});b.wbg.__wbg_new_8f67e318f15d7254=(a=>{const b=new R(c(a));return o(b)});b.wbg.__wbg_set_2357bf09366ee480=((a,b,d)=>{c(a).set(c(b),d>>>Q)});b.wbg.__wbg_length_1d25fa9e4ac21ce7=(a=>{const b=c(a).length;return b});b.wbg.__wbg_instanceof_Uint8Array_bced6f43aed8c1aa=(a=>{let b;try{b=c(a) instanceof R}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_newwithlength_6c2df9e2f3028c43=(a=>{const b=new R(a>>>Q);return o(b)});b.wbg.__wbg_subarray_2e940e41c0f5a1d9=((a,b,d)=>{const e=c(a).subarray(b>>>Q,d>>>Q);return o(e)});b.wbg.__wbg_set_759f75cd92b612d2=function(){return C(((a,b,d)=>{const e=Reflect.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=u(c(d));const f=l(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;n()[b/a0+ W]=h;n()[b/a0+ Q]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new U(r(a,b))});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return o(b)});b.wbg.__wbindgen_closure_wrapper1171=((a,b,c)=>{const d=v(a,b,515,y);return o(d)});b.wbg.__wbindgen_closure_wrapper1376=((a,b,c)=>{const d=z(a,b,588,A);return o(d)});b.wbg.__wbindgen_closure_wrapper1446=((a,b,c)=>{const d=z(a,b,614,B);return o(d)});return b});var n=(()=>{if(m===P||m.byteLength===Q){m=new Int32Array(a.memory.buffer)};return m});var p=(a=>a===O||a===P);var c=(a=>b[a]);var E=(()=>{if(D===P||D.byteLength===Q){D=new Uint32Array(a.memory.buffer)};return D});var B=((c,d,e)=>{try{a._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h4722a61981d7d29b(c,d,x(e))}finally{b[w++]=O}});var A=((b,c,d)=>{a.wasm_bindgen__convert__closures__invoke1_mut__hc48965a5ec4c1f31(b,c,o(d))});var t=(()=>{if(s===P||s.byteLength===Q){s=new Float64Array(a.memory.buffer)};return s});var v=((b,c,d,e)=>{const f={a:b,b:c,cnt:W,dtor:d};const g=(...b)=>{f.cnt++;try{return e(f.a,f.b,...b)}finally{if(--f.cnt===Q){a.__wbindgen_export_2.get(f.dtor)(f.a,f.b);f.a=Q}}};g.original=f;return g});var e=(a=>{if(a<132)return;b[a]=d;d=a});var z=((b,c,d,e)=>{const f={a:b,b:c,cnt:W,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=Q;try{return e(c,f.b,...b)}finally{if(--f.cnt===Q){a.__wbindgen_export_2.get(f.dtor)(c,f.b)}else{f.a=c}}};g.original=f;return g});var l=((a,b,c)=>{if(c===O){const c=j.encode(a);const d=b(c.length,W)>>>Q;i().subarray(d,d+ c.length).set(c);g=c.length;return d};let d=a.length;let e=b(d,W)>>>Q;const f=i();let h=Q;for(;h<d;h++){const b=a.charCodeAt(h);if(b>127)break;f[e+ h]=b};if(h!==d){if(h!==Q){a=a.slice(h)};e=c(e,d,d=h+ a.length*3,W)>>>Q;const b=i().subarray(e+ h,e+ d);const f=k(a,b);h+=f.written};g=h;return e});var i=(()=>{if(h===P||h.byteLength===Q){h=new R(a.memory.buffer)};return h});var r=((a,b)=>{a=a>>>Q;return q.decode(i().subarray(a,a+ b))});var x=(a=>{if(w==W)throw new U(`out of js stack`);b[--w]=a;return w});let a;const b=new M(N).fill(O);b.push(O,P,!0,!1);let d=b.length;let g=Q;let h=P;const j=typeof TextEncoder!==S?new TextEncoder(T):{encode:()=>{throw U(`TextEncoder not available`)}};const k=typeof j.encodeInto===V?((a,b)=>j.encodeInto(a,b)):((a,b)=>{const c=j.encode(a);b.set(c);return {read:a.length,written:c.length}});let m=P;const q=typeof TextDecoder!==S?new TextDecoder(T,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw U(`TextDecoder not available`)}};if(typeof TextDecoder!==S){q.decode()};let s=P;let w=N;let D=P;export default L;export{K as initSync}