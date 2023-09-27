/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
/******/ (() => { // webpackBootstrap
/******/ 	var __webpack_modules__ = ({

/***/ "./bootstrap.js":
/*!**********************!*\
  !*** ./bootstrap.js ***!
  \**********************/
/***/ ((__unused_webpack_module, __unused_webpack_exports, __webpack_require__) => {

eval("// A dependency graph that contains any wasm must all be imported\n// asynchronously. This `bootstrap.js` file does the single async import, so\n// that no one else needs to worry about it again.\n__webpack_require__.e(/*! import() */ \"index_js\").then(__webpack_require__.bind(__webpack_require__, /*! ./index.js */ \"./index.js\"))\n  .catch(e => console.error(\"Error importing `index.js`:\", e));\n\n\n//# sourceURL=webpack://create-wasm-app/./bootstrap.js?");

/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			id: moduleId,
/******/ 			loaded: false,
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Flag the module as loaded
/******/ 		module.loaded = true;
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = __webpack_modules__;
/******/ 	
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = __webpack_module_cache__;
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/define property getters */
/******/ 	(() => {
/******/ 		// define getter functions for harmony exports
/******/ 		__webpack_require__.d = (exports, definition) => {
/******/ 			for(var key in definition) {
/******/ 				if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/ensure chunk */
/******/ 	(() => {
/******/ 		__webpack_require__.f = {};
/******/ 		// This file contains only the entry chunk.
/******/ 		// The chunk loading function for additional chunks
/******/ 		__webpack_require__.e = (chunkId) => {
/******/ 			return Promise.all(Object.keys(__webpack_require__.f).reduce((promises, key) => {
/******/ 				__webpack_require__.f[key](chunkId, promises);
/******/ 				return promises;
/******/ 			}, []));
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/get javascript chunk filename */
/******/ 	(() => {
/******/ 		// This function allow to reference async chunks
/******/ 		__webpack_require__.u = (chunkId) => {
/******/ 			// return url for filenames based on template
/******/ 			return "" + chunkId + ".bootstrap.js";
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/global */
/******/ 	(() => {
/******/ 		__webpack_require__.g = (function() {
/******/ 			if (typeof globalThis === 'object') return globalThis;
/******/ 			try {
/******/ 				return this || new Function('return this')();
/******/ 			} catch (e) {
/******/ 				if (typeof window === 'object') return window;
/******/ 			}
/******/ 		})();
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/harmony module decorator */
/******/ 	(() => {
/******/ 		__webpack_require__.hmd = (module) => {
/******/ 			module = Object.create(module);
/******/ 			if (!module.children) module.children = [];
/******/ 			Object.defineProperty(module, 'exports', {
/******/ 				enumerable: true,
/******/ 				set: () => {
/******/ 					throw new Error('ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: ' + module.id);
/******/ 				}
/******/ 			});
/******/ 			return module;
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	(() => {
/******/ 		__webpack_require__.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop))
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/load script */
/******/ 	(() => {
/******/ 		var inProgress = {};
/******/ 		var dataWebpackPrefix = "create-wasm-app:";
/******/ 		// loadScript function to load a script via script tag
/******/ 		__webpack_require__.l = (url, done, key, chunkId) => {
/******/ 			if(inProgress[url]) { inProgress[url].push(done); return; }
/******/ 			var script, needAttach;
/******/ 			if(key !== undefined) {
/******/ 				var scripts = document.getElementsByTagName("script");
/******/ 				for(var i = 0; i < scripts.length; i++) {
/******/ 					var s = scripts[i];
/******/ 					if(s.getAttribute("src") == url || s.getAttribute("data-webpack") == dataWebpackPrefix + key) { script = s; break; }
/******/ 				}
/******/ 			}
/******/ 			if(!script) {
/******/ 				needAttach = true;
/******/ 				script = document.createElement('script');
/******/ 		
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.setAttribute("data-webpack", dataWebpackPrefix + key);
/******/ 				script.src = url;
/******/ 			}
/******/ 			inProgress[url] = [done];
/******/ 			var onScriptComplete = (prev, event) => {
/******/ 				// avoid mem leaks in IE.
/******/ 				script.onerror = script.onload = null;
/******/ 				clearTimeout(timeout);
/******/ 				var doneFns = inProgress[url];
/******/ 				delete inProgress[url];
/******/ 				script.parentNode && script.parentNode.removeChild(script);
/******/ 				doneFns && doneFns.forEach((fn) => (fn(event)));
/******/ 				if(prev) return prev(event);
/******/ 			}
/******/ 			;
/******/ 			var timeout = setTimeout(onScriptComplete.bind(null, undefined, { type: 'timeout', target: script }), 120000);
/******/ 			script.onerror = onScriptComplete.bind(null, script.onerror);
/******/ 			script.onload = onScriptComplete.bind(null, script.onload);
/******/ 			needAttach && document.head.appendChild(script);
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	(() => {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = (exports) => {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/publicPath */
/******/ 	(() => {
/******/ 		var scriptUrl;
/******/ 		if (__webpack_require__.g.importScripts) scriptUrl = __webpack_require__.g.location + "";
/******/ 		var document = __webpack_require__.g.document;
/******/ 		if (!scriptUrl && document) {
/******/ 			if (document.currentScript)
/******/ 				scriptUrl = document.currentScript.src
/******/ 			if (!scriptUrl) {
/******/ 				var scripts = document.getElementsByTagName("script");
/******/ 				if(scripts.length) scriptUrl = scripts[scripts.length - 1].src
/******/ 			}
/******/ 		}
/******/ 		// When supporting browsers where an automatic publicPath is not supported you must specify an output.publicPath manually via configuration
/******/ 		// or pass an empty string ("") and set the __webpack_public_path__ variable from your code to use your own logic.
/******/ 		if (!scriptUrl) throw new Error("Automatic publicPath is not supported in this browser");
/******/ 		scriptUrl = scriptUrl.replace(/#.*$/, "").replace(/\?.*$/, "").replace(/\/[^\/]+$/, "/");
/******/ 		__webpack_require__.p = scriptUrl;
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/jsonp chunk loading */
/******/ 	(() => {
/******/ 		// no baseURI
/******/ 		
/******/ 		// object to store loaded and loading chunks
/******/ 		// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 		// [resolve, reject, Promise] = chunk loading, 0 = chunk loaded
/******/ 		var installedChunks = {
/******/ 			"main": 0
/******/ 		};
/******/ 		
/******/ 		__webpack_require__.f.j = (chunkId, promises) => {
/******/ 				// JSONP chunk loading for javascript
/******/ 				var installedChunkData = __webpack_require__.o(installedChunks, chunkId) ? installedChunks[chunkId] : undefined;
/******/ 				if(installedChunkData !== 0) { // 0 means "already installed".
/******/ 		
/******/ 					// a Promise means "currently loading".
/******/ 					if(installedChunkData) {
/******/ 						promises.push(installedChunkData[2]);
/******/ 					} else {
/******/ 						if(true) { // all chunks have JS
/******/ 							// setup Promise in chunk cache
/******/ 							var promise = new Promise((resolve, reject) => (installedChunkData = installedChunks[chunkId] = [resolve, reject]));
/******/ 							promises.push(installedChunkData[2] = promise);
/******/ 		
/******/ 							// start chunk loading
/******/ 							var url = __webpack_require__.p + __webpack_require__.u(chunkId);
/******/ 							// create error before stack unwound to get useful stacktrace later
/******/ 							var error = new Error();
/******/ 							var loadingEnded = (event) => {
/******/ 								if(__webpack_require__.o(installedChunks, chunkId)) {
/******/ 									installedChunkData = installedChunks[chunkId];
/******/ 									if(installedChunkData !== 0) installedChunks[chunkId] = undefined;
/******/ 									if(installedChunkData) {
/******/ 										var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 										var realSrc = event && event.target && event.target.src;
/******/ 										error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 										error.name = 'ChunkLoadError';
/******/ 										error.type = errorType;
/******/ 										error.request = realSrc;
/******/ 										installedChunkData[1](error);
/******/ 									}
/******/ 								}
/******/ 							};
/******/ 							__webpack_require__.l(url, loadingEnded, "chunk-" + chunkId, chunkId);
/******/ 						} else installedChunks[chunkId] = 0;
/******/ 					}
/******/ 				}
/******/ 		};
/******/ 		
/******/ 		// no prefetching
/******/ 		
/******/ 		// no preloaded
/******/ 		
/******/ 		// no HMR
/******/ 		
/******/ 		// no HMR manifest
/******/ 		
/******/ 		// no on chunks loaded
/******/ 		
/******/ 		// install a JSONP callback for chunk loading
/******/ 		var webpackJsonpCallback = (parentChunkLoadingFunction, data) => {
/******/ 			var [chunkIds, moreModules, runtime] = data;
/******/ 			// add "moreModules" to the modules object,
/******/ 			// then flag all "chunkIds" as loaded and fire callback
/******/ 			var moduleId, chunkId, i = 0;
/******/ 			if(chunkIds.some((id) => (installedChunks[id] !== 0))) {
/******/ 				for(moduleId in moreModules) {
/******/ 					if(__webpack_require__.o(moreModules, moduleId)) {
/******/ 						__webpack_require__.m[moduleId] = moreModules[moduleId];
/******/ 					}
/******/ 				}
/******/ 				if(runtime) var result = runtime(__webpack_require__);
/******/ 			}
/******/ 			if(parentChunkLoadingFunction) parentChunkLoadingFunction(data);
/******/ 			for(;i < chunkIds.length; i++) {
/******/ 				chunkId = chunkIds[i];
/******/ 				if(__webpack_require__.o(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 					installedChunks[chunkId][0]();
/******/ 				}
/******/ 				installedChunks[chunkId] = 0;
/******/ 			}
/******/ 		
/******/ 		}
/******/ 		
/******/ 		var chunkLoadingGlobal = self["webpackChunkcreate_wasm_app"] = self["webpackChunkcreate_wasm_app"] || [];
/******/ 		chunkLoadingGlobal.forEach(webpackJsonpCallback.bind(null, 0));
/******/ 		chunkLoadingGlobal.push = webpackJsonpCallback.bind(null, chunkLoadingGlobal.push.bind(chunkLoadingGlobal));
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/wasm chunk loading */
/******/ 	(() => {
/******/ 		// object to store loaded and loading wasm modules
/******/ 		var installedWasmModules = {};
/******/ 		
/******/ 		function promiseResolve() { return Promise.resolve(); }
/******/ 		
/******/ 		var wasmImportedFuncCache0;
/******/ 		var wasmImportedFuncCache1;
/******/ 		var wasmImportedFuncCache2;
/******/ 		var wasmImportedFuncCache3;
/******/ 		var wasmImportedFuncCache4;
/******/ 		var wasmImportedFuncCache5;
/******/ 		var wasmImportedFuncCache6;
/******/ 		var wasmImportedFuncCache7;
/******/ 		var wasmImportedFuncCache8;
/******/ 		var wasmImportedFuncCache9;
/******/ 		var wasmImportedFuncCache10;
/******/ 		var wasmImportedFuncCache11;
/******/ 		var wasmImportedFuncCache12;
/******/ 		var wasmImportedFuncCache13;
/******/ 		var wasmImportedFuncCache14;
/******/ 		var wasmImportedFuncCache15;
/******/ 		var wasmImportedFuncCache16;
/******/ 		var wasmImportedFuncCache17;
/******/ 		var wasmImportedFuncCache18;
/******/ 		var wasmImportedFuncCache19;
/******/ 		var wasmImportedFuncCache20;
/******/ 		var wasmImportedFuncCache21;
/******/ 		var wasmImportedFuncCache22;
/******/ 		var wasmImportedFuncCache23;
/******/ 		var wasmImportedFuncCache24;
/******/ 		var wasmImportedFuncCache25;
/******/ 		var wasmImportedFuncCache26;
/******/ 		var wasmImportedFuncCache27;
/******/ 		var wasmImportedFuncCache28;
/******/ 		var wasmImportedFuncCache29;
/******/ 		var wasmImportedFuncCache30;
/******/ 		var wasmImportedFuncCache31;
/******/ 		var wasmImportedFuncCache32;
/******/ 		var wasmImportedFuncCache33;
/******/ 		var wasmImportedFuncCache34;
/******/ 		var wasmImportedFuncCache35;
/******/ 		var wasmImportedFuncCache36;
/******/ 		var wasmImportedFuncCache37;
/******/ 		var wasmImportedFuncCache38;
/******/ 		var wasmImportedFuncCache39;
/******/ 		var wasmImportedFuncCache40;
/******/ 		var wasmImportedFuncCache41;
/******/ 		var wasmImportedFuncCache42;
/******/ 		var wasmImportedFuncCache43;
/******/ 		var wasmImportedFuncCache44;
/******/ 		var wasmImportedFuncCache45;
/******/ 		var wasmImportedFuncCache46;
/******/ 		var wasmImportedFuncCache47;
/******/ 		var wasmImportObjects = {
/******/ 			"../pkg/fallingrust_bg.wasm": function() {
/******/ 				return {
/******/ 					"./fallingrust_bg.js": {
/******/ 						"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 							if(wasmImportedFuncCache0 === undefined) wasmImportedFuncCache0 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache0["__wbindgen_object_drop_ref"](p0i32);
/******/ 						},
/******/ 						"__wbindgen_string_new": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache1 === undefined) wasmImportedFuncCache1 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache1["__wbindgen_string_new"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_log_421388ea5cf05a4e": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache2 === undefined) wasmImportedFuncCache2 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache2["__wbg_log_421388ea5cf05a4e"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_instanceof_Window_434ce1849eb4e0fc": function(p0i32) {
/******/ 							if(wasmImportedFuncCache3 === undefined) wasmImportedFuncCache3 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache3["__wbg_instanceof_Window_434ce1849eb4e0fc"](p0i32);
/******/ 						},
/******/ 						"__wbg_document_5edd43643d1060d9": function(p0i32) {
/******/ 							if(wasmImportedFuncCache4 === undefined) wasmImportedFuncCache4 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache4["__wbg_document_5edd43643d1060d9"](p0i32);
/******/ 						},
/******/ 						"__wbg_setInterval_fcb622396840b607": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache5 === undefined) wasmImportedFuncCache5 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache5["__wbg_setInterval_fcb622396840b607"](p0i32,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_createElement_d017b8d2af99bab9": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache6 === undefined) wasmImportedFuncCache6 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache6["__wbg_createElement_d017b8d2af99bab9"](p0i32,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_getElementById_b30e88aff96f66a1": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache7 === undefined) wasmImportedFuncCache7 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache7["__wbg_getElementById_b30e88aff96f66a1"](p0i32,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_clientX_e1acb81f805aa48a": function(p0i32) {
/******/ 							if(wasmImportedFuncCache8 === undefined) wasmImportedFuncCache8 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache8["__wbg_clientX_e1acb81f805aa48a"](p0i32);
/******/ 						},
/******/ 						"__wbg_clientY_9a3d223c31b7b5c6": function(p0i32) {
/******/ 							if(wasmImportedFuncCache9 === undefined) wasmImportedFuncCache9 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache9["__wbg_clientY_9a3d223c31b7b5c6"](p0i32);
/******/ 						},
/******/ 						"__wbg_get_a765dab923455e0d": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache10 === undefined) wasmImportedFuncCache10 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache10["__wbg_get_a765dab923455e0d"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_setinnerHTML_c80d74e59f460154": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache11 === undefined) wasmImportedFuncCache11 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache11["__wbg_setinnerHTML_c80d74e59f460154"](p0i32,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_instanceof_HtmlElement_d3e8f1c1d6788b24": function(p0i32) {
/******/ 							if(wasmImportedFuncCache12 === undefined) wasmImportedFuncCache12 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache12["__wbg_instanceof_HtmlElement_d3e8f1c1d6788b24"](p0i32);
/******/ 						},
/******/ 						"__wbg_setonclick_8377bc153bfeed7f": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache13 === undefined) wasmImportedFuncCache13 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache13["__wbg_setonclick_8377bc153bfeed7f"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_setonmousedown_25160aa8fdcae0fd": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache14 === undefined) wasmImportedFuncCache14 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache14["__wbg_setonmousedown_25160aa8fdcae0fd"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_setonmousemove_c2edcd253a0e0ff0": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache15 === undefined) wasmImportedFuncCache15 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache15["__wbg_setonmousemove_c2edcd253a0e0ff0"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_setonmouseup_22ec6a73ac13b97d": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache16 === undefined) wasmImportedFuncCache16 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache16["__wbg_setonmouseup_22ec6a73ac13b97d"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_setontouchstart_1da9747853fec4c2": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache17 === undefined) wasmImportedFuncCache17 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache17["__wbg_setontouchstart_1da9747853fec4c2"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_setontouchend_f4f79b2bc963340f": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache18 === undefined) wasmImportedFuncCache18 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache18["__wbg_setontouchend_f4f79b2bc963340f"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_setontouchmove_5b4b4d7ac396366c": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache19 === undefined) wasmImportedFuncCache19 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache19["__wbg_setontouchmove_5b4b4d7ac396366c"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_instanceof_CanvasRenderingContext2d_e8b3a478a1b32d55": function(p0i32) {
/******/ 							if(wasmImportedFuncCache20 === undefined) wasmImportedFuncCache20 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache20["__wbg_instanceof_CanvasRenderingContext2d_e8b3a478a1b32d55"](p0i32);
/******/ 						},
/******/ 						"__wbg_setfillStyle_680401a47926faa5": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache21 === undefined) wasmImportedFuncCache21 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache21["__wbg_setfillStyle_680401a47926faa5"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_beginPath_5a6fcb370b5b8eb3": function(p0i32) {
/******/ 							if(wasmImportedFuncCache22 === undefined) wasmImportedFuncCache22 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache22["__wbg_beginPath_5a6fcb370b5b8eb3"](p0i32);
/******/ 						},
/******/ 						"__wbg_fill_0ced47ff22561640": function(p0i32) {
/******/ 							if(wasmImportedFuncCache23 === undefined) wasmImportedFuncCache23 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache23["__wbg_fill_0ced47ff22561640"](p0i32);
/******/ 						},
/******/ 						"__wbg_rect_aae02820ff6b1267": function(p0i32,p1f64,p2f64,p3f64,p4f64) {
/******/ 							if(wasmImportedFuncCache24 === undefined) wasmImportedFuncCache24 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache24["__wbg_rect_aae02820ff6b1267"](p0i32,p1f64,p2f64,p3f64,p4f64);
/******/ 						},
/******/ 						"__wbg_clearRect_748c58481ac89a96": function(p0i32,p1f64,p2f64,p3f64,p4f64) {
/******/ 							if(wasmImportedFuncCache25 === undefined) wasmImportedFuncCache25 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache25["__wbg_clearRect_748c58481ac89a96"](p0i32,p1f64,p2f64,p3f64,p4f64);
/******/ 						},
/******/ 						"__wbg_offsetX_8bfa4f66ce658903": function(p0i32) {
/******/ 							if(wasmImportedFuncCache26 === undefined) wasmImportedFuncCache26 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache26["__wbg_offsetX_8bfa4f66ce658903"](p0i32);
/******/ 						},
/******/ 						"__wbg_offsetY_5694fb49f178196d": function(p0i32) {
/******/ 							if(wasmImportedFuncCache27 === undefined) wasmImportedFuncCache27 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache27["__wbg_offsetY_5694fb49f178196d"](p0i32);
/******/ 						},
/******/ 						"__wbg_appendChild_3fe5090c665d3bb4": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache28 === undefined) wasmImportedFuncCache28 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache28["__wbg_appendChild_3fe5090c665d3bb4"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_targetTouches_ef46cf1c85563f91": function(p0i32) {
/******/ 							if(wasmImportedFuncCache29 === undefined) wasmImportedFuncCache29 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache29["__wbg_targetTouches_ef46cf1c85563f91"](p0i32);
/******/ 						},
/******/ 						"__wbg_instanceof_HtmlCanvasElement_a6157e470d06b638": function(p0i32) {
/******/ 							if(wasmImportedFuncCache30 === undefined) wasmImportedFuncCache30 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache30["__wbg_instanceof_HtmlCanvasElement_a6157e470d06b638"](p0i32);
/******/ 						},
/******/ 						"__wbg_setwidth_362e8db8cbadbe96": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache31 === undefined) wasmImportedFuncCache31 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache31["__wbg_setwidth_362e8db8cbadbe96"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_setheight_28f53831182cc410": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache32 === undefined) wasmImportedFuncCache32 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache32["__wbg_setheight_28f53831182cc410"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_getContext_bd4e9445094eda84": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache33 === undefined) wasmImportedFuncCache33 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache33["__wbg_getContext_bd4e9445094eda84"](p0i32,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_newnoargs_f579424187aa1717": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache34 === undefined) wasmImportedFuncCache34 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache34["__wbg_newnoargs_f579424187aa1717"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_call_89558c3e96703ca1": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache35 === undefined) wasmImportedFuncCache35 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache35["__wbg_call_89558c3e96703ca1"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbindgen_object_clone_ref": function(p0i32) {
/******/ 							if(wasmImportedFuncCache36 === undefined) wasmImportedFuncCache36 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache36["__wbindgen_object_clone_ref"](p0i32);
/******/ 						},
/******/ 						"__wbg_self_e23d74ae45fb17d1": function() {
/******/ 							if(wasmImportedFuncCache37 === undefined) wasmImportedFuncCache37 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache37["__wbg_self_e23d74ae45fb17d1"]();
/******/ 						},
/******/ 						"__wbg_window_b4be7f48b24ac56e": function() {
/******/ 							if(wasmImportedFuncCache38 === undefined) wasmImportedFuncCache38 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache38["__wbg_window_b4be7f48b24ac56e"]();
/******/ 						},
/******/ 						"__wbg_globalThis_d61b1f48a57191ae": function() {
/******/ 							if(wasmImportedFuncCache39 === undefined) wasmImportedFuncCache39 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache39["__wbg_globalThis_d61b1f48a57191ae"]();
/******/ 						},
/******/ 						"__wbg_global_e7669da72fd7f239": function() {
/******/ 							if(wasmImportedFuncCache40 === undefined) wasmImportedFuncCache40 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache40["__wbg_global_e7669da72fd7f239"]();
/******/ 						},
/******/ 						"__wbindgen_is_undefined": function(p0i32) {
/******/ 							if(wasmImportedFuncCache41 === undefined) wasmImportedFuncCache41 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache41["__wbindgen_is_undefined"](p0i32);
/******/ 						},
/******/ 						"__wbg_random_7b8246250fd79f60": function() {
/******/ 							if(wasmImportedFuncCache42 === undefined) wasmImportedFuncCache42 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache42["__wbg_random_7b8246250fd79f60"]();
/******/ 						},
/******/ 						"__wbindgen_debug_string": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache43 === undefined) wasmImportedFuncCache43 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache43["__wbindgen_debug_string"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache44 === undefined) wasmImportedFuncCache44 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache44["__wbindgen_throw"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbindgen_closure_wrapper83": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache45 === undefined) wasmImportedFuncCache45 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache45["__wbindgen_closure_wrapper83"](p0i32,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbindgen_closure_wrapper85": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache46 === undefined) wasmImportedFuncCache46 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache46["__wbindgen_closure_wrapper85"](p0i32,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbindgen_closure_wrapper87": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache47 === undefined) wasmImportedFuncCache47 = __webpack_require__.c["../pkg/fallingrust_bg.js"].exports;
/******/ 							return wasmImportedFuncCache47["__wbindgen_closure_wrapper87"](p0i32,p1i32,p2i32);
/******/ 						}
/******/ 					}
/******/ 				};
/******/ 			},
/******/ 		};
/******/ 		
/******/ 		var wasmModuleMap = {
/******/ 			"index_js": [
/******/ 				"../pkg/fallingrust_bg.wasm"
/******/ 			]
/******/ 		};
/******/ 		
/******/ 		// object with all WebAssembly.instance exports
/******/ 		__webpack_require__.w = {};
/******/ 		
/******/ 		// Fetch + compile chunk loading for webassembly
/******/ 		__webpack_require__.f.wasm = function(chunkId, promises) {
/******/ 		
/******/ 			var wasmModules = wasmModuleMap[chunkId] || [];
/******/ 		
/******/ 			wasmModules.forEach(function(wasmModuleId, idx) {
/******/ 				var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/ 		
/******/ 				// a Promise means "currently loading" or "already loaded".
/******/ 				if(installedWasmModuleData)
/******/ 					promises.push(installedWasmModuleData);
/******/ 				else {
/******/ 					var importObject = wasmImportObjects[wasmModuleId]();
/******/ 					var req = fetch(__webpack_require__.p + "" + {"index_js":{"../pkg/fallingrust_bg.wasm":"ef502649a896bac35ffc"}}[chunkId][wasmModuleId] + ".module.wasm");
/******/ 					var promise;
/******/ 					if(importObject && typeof importObject.then === 'function' && typeof WebAssembly.compileStreaming === 'function') {
/******/ 						promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 							return WebAssembly.instantiate(items[0], items[1]);
/******/ 						});
/******/ 					} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 						promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 					} else {
/******/ 						var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 						promise = bytesPromise.then(function(bytes) {
/******/ 							return WebAssembly.instantiate(bytes, importObject);
/******/ 						});
/******/ 					}
/******/ 					promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 						return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 					}));
/******/ 				}
/******/ 			});
/******/ 		};
/******/ 	})();
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// module cache are used so entry inlining is disabled
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	var __webpack_exports__ = __webpack_require__("./bootstrap.js");
/******/ 	
/******/ })()
;