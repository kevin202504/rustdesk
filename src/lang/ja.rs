lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "状態"),
        ("Your Desktop", "あなたのコンピューター"),
        ("desk_tip", "下記のIDとパスワードであなたのコンピューターにアクセスできます。"),
        ("Password", "パスワード"),
        ("Ready", "準備完了"),
        ("Established", "接続完了"),
        ("connecting_status", "RuskDeskネットワークに接続中..."),
        ("Enable service", "サービスを有効化"),
        ("Start service", "サービスを開始"),
        ("Service is running", "サービスが実行されています"),
        ("Service is not running", "サービスは停止しています"),
        ("not_ready_status", "接続できません。ネットワーク接続を確認してください"),
        ("Control Remote Desktop", "リモートコンピューターを操作"),
        ("Transfer file", "ファイル転送"),
        ("Connect", "接続"),
        ("Recent sessions", "最近のセッション"),
        ("Address book", "アドレス帳"),
        ("Confirmation", "確認"),
        ("TCP tunneling", "TCPトンネリング"),
        ("Remove", "削除"),
        ("Refresh random password", "ランダムパスワードを再生成"),
        ("Set your own password", "パスワードを設定"),
        ("Enable keyboard/mouse", "キーボード/マウスを有効化"),
        ("Enable clipboard", "クリップボードを有効化"),
        ("Enable file transfer", "ファイル転送を有効化"),
        ("Enable TCP tunneling", "TCPトンネリングを有効化"),
        ("IP Whitelisting", "IPホワイトリスト"),
        ("ID/Relay Server", "認証/中継サーバー"),
        ("Import server config", "サーバー設定をインポート"),
        ("Export Server Config", "サーバー設定をエクスポート"),
        ("Import server configuration successfully", "サーバー設定のインポートに成功しました"),
        ("Export server configuration successfully", "サーバー設定のエクスポートに成功しました"),
        ("Invalid server configuration", "無効なサーバー設定です"),
        ("Clipboard is empty", "クリップボードは空です"),
        ("Stop service", "サービスを停止"),
        ("Change ID", "IDを変更"),
        ("Your new ID", "新しいID"),
        ("length %min% to %max%", "長さが%min%～%max%文字"),
        ("starts with a letter", "始まりがアルファベット"),
        ("allowed characters", "使用可能な文字のみ"),
        ("id_change_tip", "使用できるのは大文字・小文字のアルファベット、数字、アンダースコア（_）のみです。先頭の文字はアルファベット、長さは6文字から16文字である必要があります。"),
        ("Website", "公式サイト"),
        ("About", "RustDeskについて"),
        ("Slogan_tip", "この混沌とした世界から、愛をこめて！"),
        ("Privacy Statement", "プライバシーポリシー"),
        ("Mute", "ミュート"),
        ("Build Date", "ビルド日時"),
        ("Version", "バージョン"),
        ("Home", "ホーム"),
        ("Audio Input", "音声入力"),
        ("Enhancements", "拡張機能"),
        ("Hardware Codec", "ハードウェアコーデック"),
        ("Adaptive bitrate", "可変ビットレート"),
        ("ID Server", "認証サーバー"),
        ("Relay Server", "中継サーバー"),
        ("API Server", "APIサーバー"),
        ("invalid_http", "http://またはhttps://から始まる必要があります。"),
        ("Invalid IP", "無効なIP"),
        ("Invalid format", "無効なフォーマット"),
        ("server_not_support", "このサーバーには現在対応していません。"),
        ("Not available", "利用不可"),
        ("Too frequent", "接続の頻度が高すぎます！"),
        ("Cancel", "キャンセル"),
        ("Skip", "スキップ"),
        ("Close", "閉じる"),
        ("Retry", "再試行"),
        ("OK", "OK"),
        ("Password Required", "パスワードが必要です"),
        ("Please enter your password", "パスワードを入力してください"),
        ("Remember password", "パスワードを記憶する"),
        ("Wrong Password", "パスワードが間違っています"),
        ("Do you want to enter again?", "もう一度入力しますか？"),
        ("Connection Error", "接続エラー"),
        ("Error", "エラー"),
        ("Reset by the peer", "リモートホストによって接続がリセットされました"),
        ("Connecting...", "接続中..."),
        ("Connection in progress. Please wait.", "接続中です。しばらくお待ちください。"),
        ("Please try 1 minute later", "1分後にもう一度お試しください"),
        ("Login Error", "ログインエラー"),
        ("Successful", "成功"),
        ("Connected, waiting for image...", "接続完了、映像を待機しています..."),
        ("Name", "名前"),
        ("Type", "種類"),
        ("Modified", "最終更新日"),
        ("Size", "サイズ"),
        ("Show Hidden Files", "隠しファイルを表示"),
        ("Receive", "受信"),
        ("Send", "送信"),
        ("Refresh File", "ファイルを更新"),
        ("Local", "ローカル"),
        ("Remote", "リモート"),
        ("Remote Computer", "リモートコンピューター"),
        ("Local Computer", "ローカルコンピューター"),
        ("Confirm Delete", "削除の確認"),
        ("Delete", "削除"),
        ("Properties", "プロパティ"),
        ("Multi Select", "複数選択"),
        ("Select All", "すべて選択"),
        ("Unselect All", "選択をすべて解除"),
        ("Empty Directory", "空のディレクトリ"),
        ("Not an empty directory", "空ではないディレクトリ"),
        ("Are you sure you want to delete this file?", "本当にファイルを削除しますか？"),
        ("Are you sure you want to delete this empty directory?", "本当に空のディレクトリを削除しますか？"),
        ("Are you sure you want to delete the file of this directory?", "本当にディレクトリ内のファイルを削除しますか？"),
        ("Do this for all conflicts", "すべてに適用する"),
        ("This is irreversible!", "この操作は元に戻せません！"),
        ("Deleting", "削除中"),
        ("files", "ファイル"),
        ("Waiting", "待機中"),
        ("Finished", "完了"),
        ("Speed", "速度"),
        ("Custom Image Quality", "画質をカスタムする"),
        ("Privacy mode", "プライバシーモード"),
        ("Block user input", "ユーザーの入力をブロック"),
        ("Unblock user input", "ユーザーの入力を許可"),
        ("Adjust Window", "ウィンドウを調整"),
        ("Original", "オリジナル"),
        ("Shrink", "縮小"),
        ("Stretch", "伸縮"),
        ("Scrollbar", "スクロールバー"),
        ("ScrollAuto", "自動スクロール"),
        ("Good image quality", "画質優先"),
        ("Balanced", "バランス"),
        ("Optimize reaction time", "速度優先"),
        ("Custom", "カスタム"),
        ("Show remote cursor", "リモートコンピューターのカーソルを表示"),
        ("Show quality monitor", "品質モニターを表示"),
        ("Disable clipboard", "クリップボードを無効化"),
        ("Lock after session end", "セッション終了後にロックする"),
        ("Insert Ctrl + Alt + Del", "Ctrl + Alt + Del 送信"),
        ("Insert Lock", "ロック命令を送信"),
        ("Refresh", "更新"),
        ("ID does not exist", "IDが存在しません"),
        ("Failed to connect to rendezvous server", "ランデブーサーバーに接続できませんでした"),
        ("Please try later", "後でもう一度お試しください"),
        ("Remote desktop is offline", "リモートコンピューターがオフラインです"),
        ("Key mismatch", "キーが一致しません"),
        ("Timeout", "タイムアウト"),
        ("Failed to connect to relay server", "中継サーバーに接続できませんでした"),
        ("Failed to connect via rendezvous server", "ランデブーサーバー経由で接続できませんでした"),
        ("Failed to connect via relay server", "中継サーバー経由で接続できませんでした"),
        ("Failed to make direct connection to remote desktop", "リモートコンピューターと直接接続できませんでした"),
        ("Set Password", "パスワードを設定"),
        ("OS Password", "OSのパスワード"),
        ("install_tip", "UACの影響により、RustDeskがリモートコンピューター上で正常に動作しない場合があります。UACを回避するには、下のボタンをクリックしてシステムにRustDeskをインストールしてください。"),
        ("Click to upgrade", "アップグレード"),
        ("Click to download", "ダウンロード"),
        ("Click to update", "アップデート"),
        ("Configure", "設定"),
        ("config_acc", "リモートからあなたのコンピューターを操作するには、RustDeskに「アクセシビリティ」権限を与える必要があります。"),
        ("config_screen", "リモートからあなたのコンピューターにアクセスするには、RustDeskに「画面録画」の権限を与える必要があります。"),
        ("Installing ...", "インストール中..."),
        ("Install", "インストール"),
        ("Installation", "インストール"),
        ("Installation Path", "インストール先のパス"),
        ("Create start menu shortcuts", "スタートメニューにショートカットを作成する"),
        ("Create desktop icon", "デスクトップにアイコンを作成する"),
        ("agreement_tip", "インストールを開始することで、ライセンス条項に同意したとみなされます。"),
        ("Accept and Install", "同意してインストール"),
        ("End-user license agreement", "エンドユーザー ライセンス条項"),
        ("Generating ...", "生成中..."),
        ("Your installation is lower version.", "インストールされているバージョンが古くなっています。"),
        ("not_close_tcp_tip", "トンネルの使用中はこのウィンドウを閉じないでください"),
        ("Listening ...", "リッスン中 ..."),
        ("Remote Host", "リモートホスト"),
        ("Remote Port", "リモートポート"),
        ("Action", "操作"),
        ("Add", "追加"),
        ("Local Port", "ローカルのポート"),
        ("Local Address", "ローカルポート"),
        ("Change Local Port", "ローカルポートを変更"),
        ("setup_server_tip", "より高速に接続したい場合は、自分のサーバーをセットアップすることをおすすめします"),
        ("Too short, at least 6 characters.", "文字数が短すぎます。最低文字数は6文字です。"),
        ("The confirmation is not identical.", "確認欄と入力が一致しません。"),
        ("Permissions", "権限"),
        ("Accept", "承諾"),
        ("Dismiss", "無視"),
        ("Disconnect", "切断"),
        ("Enable file copy and paste", "ファイルのコピーと貼り付けを許可"),
        ("Connected", "接続済み"),
        ("Direct and encrypted connection", "直接接続 接続は暗号化されています"),
        ("Relayed and encrypted connection", "中継接続 接続は暗号化されています"),
        ("Direct and unencrypted connection", "直接接続 接続が暗号化されていません"),
        ("Relayed and unencrypted connection", "中継接続 接続が暗号化されていません"),
        ("Enter Remote ID", "リモートIDを入力"),
        ("Enter your password", "パスワードを入力"),
        ("Logging in...", "ログイン中..."),
        ("Enable RDP session sharing", "RDPセッション共有を有効化"),
        ("Auto Login", "自動ログイン"),
        ("Enable direct IP access", "直接IPアクセスを有効化"),
        ("Rename", "名前の変更"),
        ("Space", "スペース"),
        ("Create desktop shortcut", "デスクトップにショートカットを作成する"),
        ("Change Path", "パスを変更"),
        ("Create Folder", "フォルダを作成"),
        ("Please enter the folder name", "フォルダ名を入力してください"),
        ("Fix it", "修復する"),
        ("Warning", "警告"),
        ("Login screen using Wayland is not supported", "Waylandを使用したログインスクリーンはサポートされていません"),
        ("Reboot required", "再起動が必要です"),
        ("Unsupported display server", "サポートされていないディスプレイサーバー"),
        ("x11 expected", "X11 が必要です"),
        ("Port", "ポート"),
        ("Settings", "設定"),
        ("Username", "ユーザー名"),
        ("Invalid port", "無効なポート"),
        ("Closed manually by the peer", "リモートホストによって切断されました"),
        ("Enable remote configuration modification", "リモート設定の変更を有効化"),
        ("Run without install", "インストールせずに実行"),
        ("Connect via relay", "中継サーバー経由で接続"),
        ("Always connect via relay", "常に中継サーバー経由で接続"),
        ("whitelist_tip", "ホワイトリストに登録されたIPからのみ接続を許可します"),
        ("Login", "ログイン"),
        ("Verify", "認証"),
        ("Remember me", "入力内容を記憶する"),
        ("Trust this device", "このデバイスを信頼する"),
        ("Verification code", "認証コード"),
        ("verification_tip", "登録されたメールアドレスに認証コードが送信されました。認証コードを入力して、ログインを続行してください。"),
        ("Logout", "ログアウト"),
        ("Tags", "タグ"),
        ("Search ID", "IDを検索"),
        ("whitelist_sep", "カンマやセミコロン、空白、改行で区切ってください"),
        ("Add ID", "IDを追加"),
        ("Add Tag", "タグを追加"),
        ("Unselect all tags", "全てのタグを選択解除"),
        ("Network error", "ネットワークエラー"),
        ("Username missed", "ユーザー名がありません"),
        ("Password missed", "パスワードがありません"),
        ("Wrong credentials", "資格情報が間違っています"),
        ("The verification code is incorrect or has expired", "認証コードが間違っているか、有効期限が切れています"),
        ("Edit Tag", "タグを編集"),
        ("Forget Password", "パスワードを忘れる"),
        ("Favorites", "お気に入り"),
        ("Add to Favorites", "お気に入りに追加"),
        ("Remove from Favorites", "お気に入りから削除"),
        ("Empty", "空"),
        ("Invalid folder name", "無効なフォルダ名"),
        ("Socks5 Proxy", "SOCKS5プロキシ"),
        ("Socks5/Http(s) Proxy", "Socks5/Http(s)プロキシ"),
        ("Discovered", "発見済み"),
        ("install_daemon_tip", "起動時にRustDeskを開始するには、システムサービスをインストールする必要があります。"),
        ("Remote ID", "リモートID"),
        ("Paste", "貼り付け"),
        ("Paste here?", "ここに貼り付けますか？"),
        ("Are you sure to close the connection?", "本当に切断しますか？"),
        ("Download new version", "新しいバージョンをダウンロード"),
        ("Touch mode", "タッチモード"),
        ("Mouse mode", "マウスモード"),
        ("One-Finger Tap", "1本指でタップ"),
        ("Left Mouse", "マウス左クリック"),
        ("One-Long Tap", "1本指でロングタップ"),
        ("Two-Finger Tap", "2本指でタップ"),
        ("Right Mouse", "マウス右クリック"),
        ("One-Finger Move", "1本指でドラッグ"),
        ("Double Tap & Move", "2本指でタップ&ドラッグ"),
        ("Mouse Drag", "マウスドラッグ"),
        ("Three-Finger vertically", "3本指で縦方向"),
        ("Mouse Wheel", "マウスホイール"),
        ("Two-Finger Move", "2本指でドラッグ"),
        ("Canvas Move", "キャンバスの移動"),
        ("Pinch to Zoom", "ピンチしてズーム"),
        ("Canvas Zoom", "キャンバスのズーム"),
        ("Reset canvas", "キャンバスのリセット"),
        ("No permission of file transfer", "ファイル転送の権限がありません"),
        ("Note", "ノート"),
        ("Connection", "接続"),
        ("Share screen", "画面を共有"),
        ("Chat", "チャット"),
        ("Total", "計"),
        ("items", "個のアイテム"),
        ("Selected", "選択済み"),
        ("Screen Capture", "画面キャプチャ"),
        ("Input Control", "入力操作"),
        ("Audio Capture", "音声キャプチャ"),
        ("Do you accept?", "許可しますか？"),
        ("Open System Setting", "システム設定を開く"),
        ("How to get Android input permission?", "Androidの入力権限を取得するには？"),
        ("android_input_permission_tip1", "このAndroid端末をリモートコンピューターからマウスやタッチで操作するには、RustDeskに「アクセシビリティ」サービスの使用を許可する必要があります。"),
        ("android_input_permission_tip2", "次の端末設定ページに進み、「インストール済みアプリ」から「RustDesk Input」を有効にしてください。"),
        ("android_new_connection_tip", "新しい操作リクエストが届きました。この端末を操作しようとしています。"),
        ("android_service_will_start_tip", "「画面キャプチャ」を有効にするとサービスが自動的に開始され、他の端末がこの端末への接続をリクエストできるようになります。"),
        ("android_stop_service_tip", "サービスを停止すると、自動的に現在のセッションがすべて閉じられます。"),
        ("android_version_audio_tip", "現在のAndroidバージョンでは音声キャプチャはサポートされていません。Android 10以降に更新してください。"),
        ("android_start_service_tip", "「サービスを開始」をタップするか、「画面キャプチャ」の許可を有効にすると、画面共有サービスが開始されます。"),
        ("android_permission_may_not_change_tip", "権限の変更は現在のセッションには適用されません。再接続後に適用されます。"),
        ("Account", "アカウント"),
        ("Overwrite", "上書き"),
        ("This file exists, skip or overwrite this file?", "このファイルは既に存在しています。スキップするか上書きしますか？"),
        ("Quit", "終了"),
        ("Help", "ヘルプ"),
        ("Failed", "失敗"),
        ("Succeeded", "成功"),
        ("Someone turns on privacy mode, exit", "プライバシーモードがオンになりました。終了します。"),
        ("Unsupported", "サポートされていません"),
        ("Peer denied", "リモートホストに拒否されました"),
        ("Please install plugins", "プラグインをインストールしてください"),
        ("Peer exit", "リモートホストが退出しました"),
        ("Failed to turn off", "オフにできませんでした"),
        ("Turned off", "オフになりました"),
        ("Language", "言語"),
        ("Keep RustDesk background service", "RustDesk バックグラウンドサービスを維持"),
        ("Ignore Battery Optimizations", "バッテリーの最適化を無効にする"),
        ("android_open_battery_optimizations_tip", "この機能を使わない場合は、RestDeskアプリの設定ページから「バッテリー」に進み、「制限なし」のチェックを外してください"),
        ("Start on boot", "起動時に自動実行する"),
        ("Start the screen sharing service on boot, requires special permissions", "起動時に画面共有サービスを開始します。これには特別な権限が必要です。"),
        ("Connection not allowed", "接続が許可されていません"),
        ("Legacy mode", "レガシーモード"),
        ("Map mode", "マップモード"),
        ("Translate mode", "変換モード"),
        ("Use permanent password", "固定パスワードを使用"),
        ("Use both passwords", "どちらのパスワードも使用"),
        ("Set permanent password", "固定パスワードを設定"),
        ("Enable remote restart", "リモートからの再起動を有効化"),
        ("Restart remote device", "リモートの端末を再起動"),
        ("Are you sure you want to restart", "本当に再起動しますか"),
        ("Restarting remote device", "リモートコンピューターを再起動中"),
        ("remote_restarting_tip", "リモートコンピューターは再起動中です。このメッセージボックスを閉じて、しばらくした後にパスワードを使用して再接続してください。"),
        ("Copied", "コピーしました"),
        ("Exit Fullscreen", "全画面表示を終了"),
        ("Fullscreen", "全画面表示"),
        ("Mobile Actions", "モバイル アクション"),
        ("Select Monitor", "モニターを選択"),
        ("Control Actions", "コントロール アクション"),
        ("Display Settings", "ディスプレイの設定"),
        ("Ratio", "比率"),
        ("Image Quality", "画質"),
        ("Scroll Style", "スクロール スタイル"),
        ("Show Toolbar", "ツールバーを表示"),
        ("Hide Toolbar", "ツールバーを隠す"),
        ("Direct Connection", "直接接続"),
        ("Relay Connection", "中継接続"),
        ("Secure Connection", "安全な接続"),
        ("Insecure Connection", "安全でない接続"),
        ("Scale original", "オリジナルサイズ"),
        ("Scale adaptive", "フィットウィンドウ"),
        ("General", "一般"),
        ("Security", "セキュリティ"),
        ("Theme", "テーマ"),
        ("Dark Theme", "ダークテーマ"),
        ("Light Theme", "ライトテーマ"),
        ("Dark", "ダーク"),
        ("Light", "ライト"),
        ("Follow System", "システム設定に従う"),
        ("Enable hardware codec", "ハードウェアコーデックを有効化"),
        ("Unlock Security Settings", "セキュリティ設定のロックを解除"),
        ("Enable audio", "オーディオを有効化"),
        ("Unlock Network Settings", "ネットワーク設定のロックを解除"),
        ("Server", "サーバー"),
        ("Direct IP Access", "直接IP接続"),
        ("Proxy", "プロキシ"),
        ("Apply", "適用"),
        ("Disconnect all devices?", "すべてのデバイスから切断しますか？"),
        ("Clear", "クリア"),
        ("Audio Input Device", "音声入力デバイス"),
        ("Use IP Whitelisting", "IPホワイトリストを使用する"),
        ("Network", "ネットワーク"),
        ("Pin Toolbar", "ツールバーをピン止め"),
        ("Unpin Toolbar", "ツールバーのピン止めを解除"),
        ("Recording", "録画"),
        ("Directory", "ディレクトリ"),
        ("Automatically record incoming sessions", "受信したセッションを自動で記録する"),
        ("Automatically record outgoing sessions", ""),
        ("Change", "変更"),
        ("Start session recording", "セッションの録画を開始"),
        ("Stop session recording", "セッションの録画を停止"),
        ("Enable recording session", "セッションの録画を有効化"),
        ("Enable LAN discovery", "LAN探索を有効化"),
        ("Deny LAN discovery", "LAN探索を拒否"),
        ("Write a message", "メッセージを書き込む"),
        ("Prompt", "必須"),
        ("Please wait for confirmation of UAC...", "UACの承認を待機しています..."),
        ("elevated_foreground_window_tip", "リモートデスクトップでフォーカスされているウィンドウの操作にはより高い権限が必要なため、マウスとキーボードが一時的に使用できなくなっています。リモートユーザーにウィンドウを最小化、または接続管理画面から権限を昇格するよう要求してください。この問題を回避するには、リモートコンピューターにRustDeskをインストールしてください。"),
        ("Disconnected", "切断しました"),
        ("Other", "その他"),
        ("Confirm before closing multiple tabs", "複数のタブを閉じる前に確認する"),
        ("Keyboard Settings", "キーボード設定"),
        ("Full Access", "フルアクセス"),
        ("Screen Share", "画面共有"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Waylandを使用するには、Ubuntu 21.04 以降のバージョンが必要です。"),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Waylandを使用するには、より新しいLinuxディストリビューションが必要です。 X11デスクトップを試すか、OSを変更してください。"),
        ("JumpLink", "View"),
        ("Please Select the screen to be shared(Operate on the peer side).", "共有する画面を選択してください(リモートコンピューターが操作します)"),
        ("Show RustDesk", "RustDeskを表示"),
        ("This PC", "このPC"),
        ("or", "または"),
        ("Continue with", "で続行"),
        ("Elevate", "昇格"),
        ("Zoom cursor", "拡大カーソル"),
        ("Accept sessions via password", "パスワードによるセッションの許可"),
        ("Accept sessions via click", "クリックによるセッションの承認"),
        ("Accept sessions via both", "両方の方法でセッションを許可する"),
        ("Please wait for the remote side to accept your session request...", "リモートコンピューターがあなたのセッション要求を受け入れるまでお待ちください..."),
        ("One-time Password", "ワンタイムパスワード"),
        ("Use one-time password", "ワンタイムパスワードを使用する"),
        ("One-time password length", "ワンタイムパスワードの長さ"),
        ("Request access to your device", "デバイスへのアクセス要求"),
        ("Hide connection management window", "接続管理画面を隠す"),
        ("hide_cm_tip", "パスワードによるセッションを許可し、固定パスワードを使用する場合にのみ、管理画面の非表示を許可する。"),
        ("wayland_experiment_tip", "Waylandのサポートは試験的なものです。無人アクセスを使用する場合はX11デスクトップをご利用ください。"),
        ("Right click to select tabs", "右クリックでタフを選択"),
        ("Skipped", "スキップ"),
        ("Add to address book", "アドレス帳に追加"),
        ("Group", "グループ"),
        ("Search", "検索"),
        ("Closed manually by web console", "Webコンソールによって閉じられました"),
        ("Local keyboard type", "キーボードのタイプ"),
        ("Select local keyboard type", "キーボードのタイプを選択"),
        ("software_render_tip", "LinuxでNvidia製のグラフィックカードを使用していると、接続後すぐにリモートウィンドウが閉じてしまう場合があります。オープンソースのNouveauドライバに切り替え、ソフトウェアレンダリングを使用するよう設定すると解決するかもしれません。(RustDeskの再起動が必要です)"),
        ("Always use software rendering", "常にソフトウェアレンダリングを使用する"),
        ("config_input", "リモートコンピューターをキーボードで操作するには、RustDeskに「入力監視」権限を与える必要があります。"),
        ("config_microphone", "リモートコンピューターと通話するには、RustDeskに「音声録音」権限を与える必要があります。"),
        ("request_elevation_tip", "リモートユーザーがいる場合は、権限の昇格をリクエストできます。"),
        ("Wait", "待機"),
        ("Elevation Error", "昇格エラー"),
        ("Ask the remote user for authentication", "リモートユーザーに認証をリクエストする"),
        ("Choose this if the remote account is administrator", "使用中のリモートコンピューター アカウントが管理者の場合はこちらを選択してください"),
        ("Transmit the username and password of administrator", "管理者のユーザー名とパスワードを送信"),
        ("still_click_uac_tip", "リモートデスクトップ ユーザーがRustDeskを実行する際に、UACを許可する必要があります。"),
        ("Request Elevation", "権限の昇格をリクエストする"),
        ("wait_accept_uac_tip", "リモートデスクトップ ユーザーがUACダイアログを許可するまでしばらくお待ちください。"),
        ("Elevate successfully", "権限の昇格に成功しました"),
        ("uppercase", "大文字"),
        ("lowercase", "小文字"),
        ("digit", "桁数"),
        ("special character", "特殊文字"),
        ("length>=8", "8文字以上"),
        ("Weak", "脆弱"),
        ("Medium", "普通"),
        ("Strong", "強力"),
        ("Switch Sides", "接続方向の切り替え"),
        ("Please confirm if you want to share your desktop?", "デスクトップの共有を許可しますか？"),
        ("Display", "ディスプレイ"),
        ("Default View Style", "デフォルトの表示スタイル"),
        ("Default Scroll Style", "デフォルトのスクロールスタイル"),
        ("Default Image Quality", "デフォルトの画質"),
        ("Default Codec", "デフォルトのコーデック"),
        ("Bitrate", "ビットレート"),
        ("FPS", "FPS"),
        ("Auto", "自動"),
        ("Other Default Options", "その他のデフォルト設定"),
        ("Voice call", "音声通話"),
        ("Text chat", "テキストチャット"),
        ("Stop voice call", "音声通話を終了"),
        ("relay_hint_tip", "直接接続が行えない場合は、リレー経由での接続をお試しください。初回接続で中継接続を行いたい場合は末尾に「/r」を付けるか、最近のセッション履歴に「常に中継サーバー経由で接続」という設定がある場合はそちらを選択してください。"),
        ("Reconnect", "再接続"),
        ("Codec", "コーデック"),
        ("Resolution", "解像度"),
        ("No transfers in progress", "進行中の転送はありません"),
        ("Set one-time password length", "ワンタイムパスワードの長さを設定する"),
        ("RDP Settings", "RDP設定"),
        ("Sort by", "並べ替え"),
        ("New Connection", "新規接続"),
        ("Restore", "復元"),
        ("Minimize", "最小"),
        ("Maximize", "最大"),
        ("Your Device", "あなたのデバイス"),
        ("empty_recent_tip", "おっと、最近のセッションは見つかりませんでした。新しい計画を練る時間です！"),
        ("empty_favorite_tip", "お気に入りのリモートコンピュータがないようですね？あなたの接続先を登録しましょう！"),
        ("empty_lan_tip", "あらら、まだ近くのコンピューターは発見できていないようです。"),
        ("empty_address_book_tip", "驚くべきことに、あなたのアドレス帳には現在コンピューターが登録されていません。"),
        ("eg: admin", "例: 管理者"),
        ("Empty Username", "空のユーザー名"),
        ("Empty Password", "空のパスワード"),
        ("Me", "あなた"),
        ("identical_file_tip", "このファイルはリモートコンピューターと同一です。"),
        ("show_monitors_tip", "ツールバーにモニターを表示します"),
        ("View Mode", "表示モード"),
        ("login_linux_tip", "Xデスクトップのセッションにログインするには、リモートコンピューターのLinuxアカウントにログインする必要があります。"),
        ("verify_rustdesk_password_tip", "RustDeskのパスワードを確認する"),
        ("remember_account_tip", "このアカウントを記憶する"),
        ("os_account_desk_tip", "このアカウントは、リモートコンピューターのOSにログインし、ヘッドレスでセッションを有効化するために使用されます。"),
        ("OS Account", "OSのアカウント"),
        ("another_user_login_title_tip", "他のユーザーがすでにログインしています"),
        ("another_user_login_text_tip", "切断しました"),
        ("xorg_not_found_title_tip", "Xorgサーバーが見つかりませんでした。"),
        ("xorg_not_found_text_tip", "Xorgをインストールしてください"),
        ("no_desktop_title_tip", "デスクトップ環境が見つかりませんでした。"),
        ("no_desktop_text_tip", "GNOMEデスクトップ環境をインストールしてください"),
        ("No need to elevate", "権限昇格の必要はありません"),
        ("System Sound", "システム音声"),
        ("Default", "デフォルト"),
        ("New RDP", "新しいRDP"),
        ("Fingerprint", "フィンガープリント"),
        ("Copy Fingerprint", "フィンガープリントをコピー"),
        ("no fingerprints", "フィンガープリントがありません"),
        ("Select a peer", "リモートコンピューターを選択"),
        ("Select peers", "複数のリモートコンピューターを選択"),
        ("Plugins", "プラグイン"),
        ("Uninstall", "アンインストール"),
        ("Update", "更新"),
        ("Enable", "有効"),
        ("Disable", "無効"),
        ("Options", "設定"),
        ("resolution_original_tip", "オリジナルの解像度"),
        ("resolution_fit_local_tip", "ローカル解像度に合わせる"),
        ("resolution_custom_tip", "カスタム解像度"),
        ("Collapse toolbar", "ツールバーを折りたたむ"),
        ("Accept and Elevate", "承認して権限を昇格する"),
        ("accept_and_elevate_btn_tooltip", "接続を受け入れた上で、UAC権限を昇格します。"),
        ("clipboard_wait_response_timeout_tip", "クリップボードのコピーがタイムアウトしました。"),
        ("Incoming connection", "接続の受信"),
        ("Outgoing connection", "接続の送信"),
        ("Exit", "終了"),
        ("Open", "開く"),
        ("logout_tip", "本当にログアウトしますか？"),
        ("Service", "サービス"),
        ("Start", "開始"),
        ("Stop", "停止"),
        ("exceed_max_devices", "管理対象のデバイスが最大数に達しました。"),
        ("Sync with recent sessions", "最近のセッションと同期"),
        ("Sort tags", "タグで並べ替え"),
        ("Open connection in new tab", "新しいタブで接続を開く"),
        ("Move tab to new window", "新しいウィンドウにタブを移動する"),
        ("Can not be empty", "空にすることはできません"),
        ("Already exists", "すでに存在します"),
        ("Change Password", "パスワードを変更"),
        ("Refresh Password", "パスワードをリフレッシュ"),
        ("ID", "ID"),
        ("Grid View", "グリッドビュー"),
        ("List View", "リストビュー"),
        ("Select", "選択"),
        ("Toggle Tags", "タグの切り替え"),
        ("pull_ab_failed_tip", "アドレス帳の更新に失敗しました"),
        ("push_ab_failed_tip", "サーバーへのアドレス帳の同期に失敗しました"),
        ("synced_peer_readded_tip", "最近セッションを行ったデバイスはアドレス帳に同期されます。"),
        ("Change Color", "色の変更"),
        ("Primary Color", "プライマリ カラー"),
        ("HSV Color", "HSVカラー"),
        ("Installation Successful!", "インストールに成功しました！"),
        ("Installation failed!", "インストールに失敗しました。"),
        ("Reverse mouse wheel", "マウスホイールを反転する"),
        ("{} sessions", "{}件のセッション"),
        ("scam_title", "あなたは詐欺にあっているかもしれません！"),
        ("scam_text1", "もし、知らない相手から電話でRustDeskのインストールやサービスの開始を依頼された場合、作業を進めずに、すぐに電話を切ってください。"),
        ("scam_text2", "相手はあなたからお金や個人情報を盗もうとする詐欺師である可能性があります。"),
        ("Don't show again", "今後表示しない"),
        ("I Agree", "同意する"),
        ("Decline", "同意しない"),
        ("Timeout in minutes", "タイムアウトまでの時間(分)"),
        ("auto_disconnect_option_tip", "ユーザーが非アクティブの場合、自動的に受信したセッションを閉じる"),
        ("Connection failed due to inactivity", "リモートデスクトップ ユーザーが非アクティブなため、接続に失敗しました"),
        ("Check for software update on startup", "起動時にソフトウェアの更新をチェック"),
        ("upgrade_rustdesk_server_pro_to_{}_tip", "RustDesk Server Proをバージョン{}以上にアップグレードしてください！"),
        ("pull_group_failed_tip", "グループの更新に失敗しました"),
        ("Filter by intersection", "交差位置でフィルター"),
        ("Remove wallpaper during incoming sessions", "セッションの受信中、デスクトップ背景を削除する"),
        ("Test", "テスト"),
        ("display_is_plugged_out_msg", "モニターが接続されていません。最初のモニターを選択してください。"),
        ("No displays", "モニターがありません"),
        ("Open in new window", "新しいウィンドウで開く"),
        ("Show displays as individual windows", "モニターを別々のウィンドウとして表示"),
        ("Use all my displays for the remote session", "すべてのディスプレイをセッションで使用する"),
        ("selinux_tip", "SELinuxが有効になっているため、RustDeskが正常に動作しない可能性があります。"),
        ("Change view", "表示変更"),
        ("Big tiles", "大きなタイル"),
        ("Small tiles", "小さなタイル"),
        ("List", "リスト"),
        ("Virtual display", "仮想モニター"),
        ("Plug out all", "すべて切断する"),
        ("True color (4:4:4)", "True color (4:4:4)"),
        ("Enable blocking user input", "ユーザー入力のブロックを有効化"),
        ("id_input_tip", "ID、IPアドレス、またはドメインとポート番号(<ドメイン>:<ポート>)を使用できます。\n他のサーバーのデバイスにアクセスしたい場合は、サーバーアドレス(<id>@<サーバーアドレス>?key=<キーの値>)を追加してください。 \n(例:9123456234@192.168.16.1:26117?key=5Qbwsde3unUcJBtrx9ZkvUmwFNoExHzpryHuPUdqlWM=)\nパブリックサーバーのデバイスに接続したい場合は、「<id>@public」のように入力してください。パブリックサーバーの場合、キーは不要です。\n\n初回接続で中継接続を行いたい場合は、「9123456234/r」のように末尾に「/r」を付けてください。"),
        ("privacy_mode_impl_mag_tip", "モード 1"),
        ("privacy_mode_impl_virtual_display_tip", "モード 2"),
        ("Enter privacy mode", "プライバシーモードを起動"),
        ("Exit privacy mode", "プライバシーモードを終了"),
        ("idd_not_support_under_win10_2004_tip", "Indirect display driverには対応していません。Windows 10 バージョン2004以降が必要です。"),
        ("input_source_1_tip", "入力ソース 1"),
        ("input_source_2_tip", "入力ソース 2"),
        ("Swap control-command key", "ctrlとcommandキーを入れ替える"),
        ("swap-left-right-mouse", "マウスのクリックを入れ替える"),
        ("2FA code", "二要素認証コード"),
        ("More", "詳細"),
        ("enable-2fa-title", "二要素認証を有効化"),
        ("enable-2fa-desc", "認証アプリをセットアップします。Authy、MicrosoftまたはGoogle AuthenticatorなどがPCまたはスマートフォンで利用できます。\n\nQRコードをスキャンし、アプリが表示するコードを入力することで二要素認証が有効になります。"),
        ("wrong-2fa-code", "コードが違います。コードと端末の時刻設定が正しいかをご確認ください。"),
        ("enter-2fa-title", "二要素認証"),
        ("Email verification code must be 6 characters.", "電子メール認証コードは6文字である必要があります。"),
        ("2FA code must be 6 digits.", "二要素認証コードは6文字である必要があります。"),
        ("Multiple Windows sessions found", "複数のWindowsセッションが見つかりました"),
        ("Please select the session you want to connect to", "接続したいセッションを選択してください"),
        ("powered_by_me", "Powered by RustDesk"),
        ("outgoing_only_desk_tip", "カスタマイズされたエディションを使用しています。\n他のコンピューターに接続できますが、他のコンピューターからのリクエストは受信できません。"),
        ("preset_password_warning", "このエディションには、デフォルトで固定パスワードが設定されています。このパスワードを知っているユーザーはあなたのデバイスを完全にコントロールできるため、そのような危険がある場合は直ちにRustDeskをアンインストールして下さい！"),
        ("Security Alert", "セキュリティ警告"),
        ("My address book", "あなたのアドレス帳"),
        ("Personal", "個人"),
        ("Owner", "所有者"),
        ("Set shared password", "共有パスワードの設定"),
        ("Exist in", "既に存在します"),
        ("Read-only", "読み取り専用"),
        ("Read/Write", "読み取り/書き込み"),
        ("Full Control", "フルアクセス"),
        ("share_warning_tip", "フィールドは共有され、他の人からも閲覧できます。"),
        ("Everyone", "全員"),
        ("ab_web_console_tip", "webコンソールの詳細"),
        ("allow-only-conn-window-open-tip", "RustDeskのウィンドウが開いている場合のみ接続を許可する"),
        ("no_need_privacy_mode_no_physical_displays_tip", "物理モニターが存在しないため、プライバシーモードは不要です。"),
        ("Follow remote cursor", "リモートカーソルに追従"),
        ("Follow remote window focus", "リモートウィンドウのフォーカスに追従"),
        ("default_proxy_tip", "デフォルトのプロトコルとポートはSocks5と1080です。"),
        ("no_audio_input_device_tip", "オーディオ入力デバイスが見つかりません。"),
        ("Incoming", "受信"),
        ("Outgoing", "発信"),
        ("Clear Wayland screen selection", "Waylandの画面選択をクリア"),
        ("clear_Wayland_screen_selection_tip", "画面選択をクリア後、共有画面を再び選択できます。"),
        ("confirm_clear_Wayland_screen_selection_tip", "本当にWaylandの画面選択をクリアしますか？"),
        ("android_new_voice_call_tip", "新しい音声通話リクエストを受信しました。承認すると音声通話に切り替わります。"),
        ("texture_render_tip", "テクスチャレンダリングを使用し、画像をより滑らかに描画します。レンダリングの問題が発生した場合は無効にしてみてください。"),
        ("Use texture rendering", "テクスチャレンダリングを使用"),
        ("Floating window", "フローティングウィンドウ"),
        ("floating_window_tip", "RustDeskのバックグラウンドサービスを維持するために使用されます。"),
        ("Keep screen on", "常に画面をオン"),
        ("Never", "画面をオンにしない"),
        ("During controlled", "操作中"),
        ("During service is on", "サービスの動作中"),
        ("Capture screen using DirectX", "DirectXを使用した画面キャプチャ"),
        ("Back", "戻る"),
        ("Apps", "アプリ"),
        ("Volume up", "音量アップ"),
        ("Volume down", "音量ダウン"),
        ("Power", "電源"),
        ("Telegram bot", "Telegram Bot"),
        ("enable-bot-tip", "この機能を有効にすると、ボットから二要素認証コードを受け取ることができます。また、接続時の通知としても機能します。"),
        ("enable-bot-desc", "1. @BotFatherのチャットを開きます。\n2. 「/newbot」コマンドを送信します。送信後、トークンを取得できます。\n3. 新しく作成したbotとチャットを開始します。「/hello」のようにスラッシュで始まるメッセージを送信して起動します。\n"),
        ("cancel-2fa-confirm-tip", "本当に二要素認証をキャンセルしますか？"),
        ("cancel-bot-confirm-tip", "本当にTelegram Botをキャンセルしますか？"),
        ("About RustDesk", "RustDeskについて"),
        ("Send clipboard keystrokes", "クリップボードの内容をキー入力として送信する"),
        ("network_error_tip", "ネットワーク接続を確認し、再度お試しください。"),
        ("Unlock with PIN", "PINでロック解除"),
        ("Requires at least {} characters", "最低でも{}文字必要です"),
        ("Wrong PIN", "PINが間違っています"),
        ("Set PIN", "PINを設定"),
        ("Enable trusted devices", "承認済デバイスを有効化"),
        ("Manage trusted devices", "承認済デバイスの管理"),
        ("Platform", "プラットフォーム"),
        ("Days remaining", "残り日数"),
        ("enable-trusted-devices-tip", "承認済デバイスで2FAチェックをスキップします。"),
        ("Parent directory", "親ディレクトリ"),
        ("Resume", "再開"),
        ("Invalid file name", "無効なファイル名"),
        ("one-way-file-transfer-tip", ""),
        ("Authentication Required", ""),
        ("Authenticate", ""),
        ("web_id_input_tip", ""),
        ("Download", ""),
        ("Upload folder", ""),
        ("Upload files", ""),
        ("Clipboard is synchronized", ""),
        ("Update client clipboard", ""),
        ("Untagged", ""),
        ("new-version-of-{}-tip", ""),
        ("Accessible devices", ""),
        ("upgrade_remote_rustdesk_client_to_{}_tip", "リモート側のRustDeskクライアントをバージョン{}以上にアップグレードしてください！"),
        ("d3d_render_tip", ""),
        ("Use D3D rendering", ""),
        ("Printer", ""),
        ("printer-os-requirement-tip", ""),
        ("printer-requires-installed-{}-client-tip", ""),
        ("printer-{}-not-installed-tip", ""),
        ("printer-{}-ready-tip", ""),
        ("Install {} Printer", ""),
        ("Outgoing Print Jobs", ""),
        ("Incoming Print Jobs", ""),
        ("Incoming Print Job", ""),
        ("use-the-default-printer-tip", ""),
        ("use-the-selected-printer-tip", ""),
        ("auto-print-tip", ""),
        ("print-incoming-job-confirm-tip", ""),
        ("remote-printing-disallowed-tile-tip", ""),
        ("remote-printing-disallowed-text-tip", ""),
        ("save-settings-tip", ""),
        ("dont-show-again-tip", ""),
        ("Take screenshot", ""),
        ("Taking screenshot", ""),
        ("screenshot-merged-screen-not-supported-tip", ""),
        ("screenshot-action-tip", ""),
        ("Save as", ""),
        ("Copy to clipboard", ""),
        ("Enable remote printer", ""),
        ("Downloading {}", ""),
        ("{} Update", ""),
        ("{}-to-update-tip", ""),
        ("download-new-version-failed-tip", ""),
        ("Auto update", ""),
        ("update-failed-check-msi-tip", ""),
        ("websocket_tip", ""),
        ("Use WebSocket", ""),
        ("Trackpad speed", ""),
        ("Default trackpad speed", ""),
        ("Numeric one-time password", ""),
        ("Enable IPv6 P2P connection", ""),
        ("Enable UDP hole punching", ""),
        ("View camera", "カメラを表示"),
        ("Enable camera", ""),
        ("No cameras", ""),
        ("view_camera_unsupported_tip", ""),
        ("Terminal", ""),
        ("Enable terminal", ""),
        ("New tab", ""),
        ("Keep terminal sessions on disconnect", ""),
    ].iter().cloned().collect();
}
