(function() {var implementors = {};
implementors["futures"] = [];
implementors["stahlnet"] = [{text:"impl <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"stahlnet/struct.Server.html\" title=\"struct stahlnet::Server\">Server</a>",synthetic:false,types:["stahlnet::server::Server"]},];
implementors["tokio_fs"] = [{text:"impl&lt;P&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_fs/struct.CreateDirFuture.html\" title=\"struct tokio_fs::CreateDirFuture\">CreateDirFuture</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt;,&nbsp;</span>",synthetic:false,types:["tokio_fs::create_dir::CreateDirFuture"]},{text:"impl&lt;P&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_fs/struct.CreateDirAllFuture.html\" title=\"struct tokio_fs::CreateDirAllFuture\">CreateDirAllFuture</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt;,&nbsp;</span>",synthetic:false,types:["tokio_fs::create_dir_all::CreateDirAllFuture"]},{text:"impl <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_fs/file/struct.CloneFuture.html\" title=\"struct tokio_fs::file::CloneFuture\">CloneFuture</a>",synthetic:false,types:["tokio_fs::file::clone::CloneFuture"]},{text:"impl&lt;P&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_fs/file/struct.CreateFuture.html\" title=\"struct tokio_fs::file::CreateFuture\">CreateFuture</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,&nbsp;</span>",synthetic:false,types:["tokio_fs::file::create::CreateFuture"]},{text:"impl <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_fs/file/struct.MetadataFuture.html\" title=\"struct tokio_fs::file::MetadataFuture\">MetadataFuture</a>",synthetic:false,types:["tokio_fs::file::metadata::MetadataFuture"]},{text:"impl&lt;P&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_fs/file/struct.OpenFuture.html\" title=\"struct tokio_fs::file::OpenFuture\">OpenFuture</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,&nbsp;</span>",synthetic:false,types:["tokio_fs::file::open::OpenFuture"]},{text:"impl <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_fs/file/struct.SeekFuture.html\" title=\"struct tokio_fs::file::SeekFuture\">SeekFuture</a>",synthetic:false,types:["tokio_fs::file::seek::SeekFuture"]},{text:"impl&lt;P, Q&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_fs/struct.HardLinkFuture.html\" title=\"struct tokio_fs::HardLinkFuture\">HardLinkFuture</a>&lt;P, Q&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Q: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt;,&nbsp;</span>",synthetic:false,types:["tokio_fs::hard_link::HardLinkFuture"]},{text:"impl&lt;P&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_fs/struct.MetadataFuture.html\" title=\"struct tokio_fs::MetadataFuture\">MetadataFuture</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,&nbsp;</span>",synthetic:false,types:["tokio_fs::metadata::MetadataFuture"]},{text:"impl&lt;P, Q&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_fs/os/unix/struct.SymlinkFuture.html\" title=\"struct tokio_fs::os::unix::SymlinkFuture\">SymlinkFuture</a>&lt;P, Q&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Q: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt;,&nbsp;</span>",synthetic:false,types:["tokio_fs::os::unix::SymlinkFuture"]},{text:"impl&lt;P:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_fs/struct.ReadFile.html\" title=\"struct tokio_fs::ReadFile\">ReadFile</a>&lt;P&gt;",synthetic:false,types:["tokio_fs::read::ReadFile"]},{text:"impl&lt;P&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_fs/struct.ReadDirFuture.html\" title=\"struct tokio_fs::ReadDirFuture\">ReadDirFuture</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,&nbsp;</span>",synthetic:false,types:["tokio_fs::read_dir::ReadDirFuture"]},{text:"impl&lt;P&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_fs/struct.ReadLinkFuture.html\" title=\"struct tokio_fs::ReadLinkFuture\">ReadLinkFuture</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt;,&nbsp;</span>",synthetic:false,types:["tokio_fs::read_link::ReadLinkFuture"]},{text:"impl&lt;P&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_fs/struct.RemoveDirFuture.html\" title=\"struct tokio_fs::RemoveDirFuture\">RemoveDirFuture</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt;,&nbsp;</span>",synthetic:false,types:["tokio_fs::remove_dir::RemoveDirFuture"]},{text:"impl&lt;P&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_fs/struct.RemoveFileFuture.html\" title=\"struct tokio_fs::RemoveFileFuture\">RemoveFileFuture</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt;,&nbsp;</span>",synthetic:false,types:["tokio_fs::remove_file::RemoveFileFuture"]},{text:"impl&lt;P, Q&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_fs/struct.RenameFuture.html\" title=\"struct tokio_fs::RenameFuture\">RenameFuture</a>&lt;P, Q&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Q: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt;,&nbsp;</span>",synthetic:false,types:["tokio_fs::rename::RenameFuture"]},{text:"impl&lt;P&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_fs/struct.SetPermissionsFuture.html\" title=\"struct tokio_fs::SetPermissionsFuture\">SetPermissionsFuture</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt;,&nbsp;</span>",synthetic:false,types:["tokio_fs::set_permissions::SetPermissionsFuture"]},{text:"impl&lt;P&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_fs/struct.SymlinkMetadataFuture.html\" title=\"struct tokio_fs::SymlinkMetadataFuture\">SymlinkMetadataFuture</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,&nbsp;</span>",synthetic:false,types:["tokio_fs::symlink_metadata::SymlinkMetadataFuture"]},{text:"impl&lt;P:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static, C:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_fs/struct.WriteFile.html\" title=\"struct tokio_fs::WriteFile\">WriteFile</a>&lt;P, C&gt;",synthetic:false,types:["tokio_fs::write::WriteFile"]},];
implementors["tokio_io"] = [{text:"impl&lt;R, W&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_io/io/struct.Copy.html\" title=\"struct tokio_io::io::Copy\">Copy</a>&lt;R, W&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"tokio_io/trait.AsyncRead.html\" title=\"trait tokio_io::AsyncRead\">AsyncRead</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;W: <a class=\"trait\" href=\"tokio_io/trait.AsyncWrite.html\" title=\"trait tokio_io::AsyncWrite\">AsyncWrite</a>,&nbsp;</span>",synthetic:false,types:["tokio_io::io::copy::Copy"]},{text:"impl&lt;A&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_io/io/struct.Flush.html\" title=\"struct tokio_io::io::Flush\">Flush</a>&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"tokio_io/trait.AsyncWrite.html\" title=\"trait tokio_io::AsyncWrite\">AsyncWrite</a>,&nbsp;</span>",synthetic:false,types:["tokio_io::io::flush::Flush"]},{text:"impl&lt;R, T&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_io/io/struct.Read.html\" title=\"struct tokio_io::io::Read\">Read</a>&lt;R, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"tokio_io/trait.AsyncRead.html\" title=\"trait tokio_io::AsyncRead\">AsyncRead</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html\" title=\"trait core::convert::AsMut\">AsMut</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt;,&nbsp;</span>",synthetic:false,types:["tokio_io::io::read::Read"]},{text:"impl&lt;A, T&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_io/io/struct.ReadExact.html\" title=\"struct tokio_io::io::ReadExact\">ReadExact</a>&lt;A, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"tokio_io/trait.AsyncRead.html\" title=\"trait tokio_io::AsyncRead\">AsyncRead</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html\" title=\"trait core::convert::AsMut\">AsMut</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt;,&nbsp;</span>",synthetic:false,types:["tokio_io::io::read_exact::ReadExact"]},{text:"impl&lt;A&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_io/io/struct.ReadToEnd.html\" title=\"struct tokio_io::io::ReadToEnd\">ReadToEnd</a>&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"tokio_io/trait.AsyncRead.html\" title=\"trait tokio_io::AsyncRead\">AsyncRead</a>,&nbsp;</span>",synthetic:false,types:["tokio_io::io::read_to_end::ReadToEnd"]},{text:"impl&lt;A&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_io/io/struct.ReadUntil.html\" title=\"struct tokio_io::io::ReadUntil\">ReadUntil</a>&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"tokio_io/trait.AsyncRead.html\" title=\"trait tokio_io::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>,&nbsp;</span>",synthetic:false,types:["tokio_io::io::read_until::ReadUntil"]},{text:"impl&lt;A&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_io/io/struct.Shutdown.html\" title=\"struct tokio_io::io::Shutdown\">Shutdown</a>&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"tokio_io/trait.AsyncWrite.html\" title=\"trait tokio_io::AsyncWrite\">AsyncWrite</a>,&nbsp;</span>",synthetic:false,types:["tokio_io::io::shutdown::Shutdown"]},{text:"impl&lt;A, T&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_io/io/struct.WriteAll.html\" title=\"struct tokio_io::io::WriteAll\">WriteAll</a>&lt;A, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"tokio_io/trait.AsyncWrite.html\" title=\"trait tokio_io::AsyncWrite\">AsyncWrite</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt;,&nbsp;</span>",synthetic:false,types:["tokio_io::io::write_all::WriteAll"]},];
implementors["tokio_reactor"] = [{text:"impl <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_reactor/struct.Shutdown.html\" title=\"struct tokio_reactor::Shutdown\">Shutdown</a>",synthetic:false,types:["tokio_reactor::background::Shutdown"]},];
implementors["tokio_sync"] = [{text:"impl&lt;T&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_sync/oneshot/struct.Receiver.html\" title=\"struct tokio_sync::oneshot::Receiver\">Receiver</a>&lt;T&gt;",synthetic:false,types:["tokio_sync::oneshot::Receiver"]},];
implementors["tokio_tcp"] = [{text:"impl <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_tcp/struct.ConnectFuture.html\" title=\"struct tokio_tcp::ConnectFuture\">ConnectFuture</a>",synthetic:false,types:["tokio_tcp::stream::ConnectFuture"]},];
implementors["tokio_threadpool"] = [{text:"impl <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_threadpool/struct.Shutdown.html\" title=\"struct tokio_threadpool::Shutdown\">Shutdown</a>",synthetic:false,types:["tokio_threadpool::shutdown::Shutdown"]},{text:"impl&lt;T, E&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_threadpool/struct.SpawnHandle.html\" title=\"struct tokio_threadpool::SpawnHandle\">SpawnHandle</a>&lt;T, E&gt;",synthetic:false,types:["tokio_threadpool::thread_pool::SpawnHandle"]},];
implementors["tokio_timer"] = [{text:"impl&lt;T&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_timer/timeout/struct.Timeout.html\" title=\"struct tokio_timer::timeout::Timeout\">Timeout</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a>,&nbsp;</span>",synthetic:false,types:["tokio_timer::timeout::Timeout"]},{text:"impl <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_timer/struct.Delay.html\" title=\"struct tokio_timer::Delay\">Delay</a>",synthetic:false,types:["tokio_timer::delay::Delay"]},];
implementors["tokio_udp"] = [{text:"impl&lt;T&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_udp/struct.SendDgram.html\" title=\"struct tokio_udp::SendDgram\">SendDgram</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt;,&nbsp;</span>",synthetic:false,types:["tokio_udp::send_dgram::SendDgram"]},{text:"impl&lt;T&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_udp/struct.RecvDgram.html\" title=\"struct tokio_udp::RecvDgram\">RecvDgram</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html\" title=\"trait core::convert::AsMut\">AsMut</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt;,&nbsp;</span>",synthetic:false,types:["tokio_udp::recv_dgram::RecvDgram"]},];
implementors["tokio_uds"] = [{text:"impl&lt;T&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_uds/struct.RecvDgram.html\" title=\"struct tokio_uds::RecvDgram\">RecvDgram</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html\" title=\"trait core::convert::AsMut\">AsMut</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt;,&nbsp;</span>",synthetic:false,types:["tokio_uds::recv_dgram::RecvDgram"]},{text:"impl&lt;T, P&gt; <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_uds/struct.SendDgram.html\" title=\"struct tokio_uds::SendDgram\">SendDgram</a>&lt;T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.slice.html\">]</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt;,&nbsp;</span>",synthetic:false,types:["tokio_uds::send_dgram::SendDgram"]},{text:"impl <a class=\"trait\" href=\"futures/future/trait.Future.html\" title=\"trait futures::future::Future\">Future</a> for <a class=\"struct\" href=\"tokio_uds/struct.ConnectFuture.html\" title=\"struct tokio_uds::ConnectFuture\">ConnectFuture</a>",synthetic:false,types:["tokio_uds::stream::ConnectFuture"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
