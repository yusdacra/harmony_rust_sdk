(function() {var implementors = {};
implementors["harmony_rust_sdk"] = [{"text":"impl&lt;T, B&gt; Service&lt;Request&lt;B&gt;&gt; for ChatServiceServer&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: ChatService,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: HttpBody + Send + Sync + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;B::Error: Into&lt;StdError&gt; + Send + 'static,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, B&gt; Service&lt;Request&lt;B&gt;&gt; for AuthServiceServer&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: AuthService,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: HttpBody + Send + Sync + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;B::Error: Into&lt;StdError&gt; + Send + 'static,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, B&gt; Service&lt;Request&lt;B&gt;&gt; for MediaProxyServiceServer&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: MediaProxyService,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: HttpBody + Send + Sync + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;B::Error: Into&lt;StdError&gt; + Send + 'static,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, B&gt; Service&lt;Request&lt;B&gt;&gt; for VoiceServiceServer&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: VoiceService,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: HttpBody + Send + Sync + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;B::Error: Into&lt;StdError&gt; + Send + 'static,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()