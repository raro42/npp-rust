009 keep: Function List /etc/machine-info stock KEY=VALUE (v0.3.600)

009 keep: Function List /etc/networks name+dotted number (v0.3.597)

009 keep: Function List /etc/ethers MAC+hostname (v0.3.596)

009 keep: Function List /etc/protocols name+number 0–255 (v0.3.594)

009 keep: Function List systemd binfmt.d register lines (v0.3.592)

009 keep: Function List systemd sysusers.d entries (v0.3.591)

009 keep: Function List /etc/sysctl.conf + sysctl.d dotted key=value (v0.3.588)

009 keep: Function List /etc/subuid + /etc/subgid entries (v0.3.587)

009 keep: Function List /etc/shadow + /etc/gshadow entries (v0.3.586)

009 keep: Function List /etc/nsswitch.conf databases + [STATUS=ACTION] (v0.3.584)

009 keep: Function List /etc/resolv.conf nameserver/search/domain/options/sortlist (v0.3.583)

009 keep: Function List /etc/hosts address+hostname entries (v0.3.582)

009 keep: Function List /etc/crypttab name/device/key/options entries (v0.3.581)

009 keep: Function List /etc/fstab UUID/LABEL/tmpfs/CIFS/NFS entries (v0.3.580)

009 keep: Function List Linux audit.rules -w/-a/-A/-D/-b/-f/-e/-r/-i/--loginuid-immutable (v0.3.574)

009 keep: Function List udev rules KERNEL/ATTR/LABEL/GOTO/RUN+= (v0.3.573)

009 keep: Function List logrotate /var/log/… { path blocks + include /etc/logrotate.d (v0.3.572)
009 keep: Function List SDDM [General]/[Theme]/[Users]/[X11]/[Wayland]/[Autologin] (v0.3.568)
009 keep: Function List GDM [daemon]/[security]/[xdmcp]/[chooser]/[debug]/[greeter] (v0.3.567)
009 keep: Function List LightDM [LightDM]/[Seat:*]/[Seat:name]/[XDMCPServer]/[VNCServer]/[SeatDefaults] (v0.3.566)
009 keep: Function List Xorg Section ServerLayout/Files/Module/InputDevice/InputClass/Device/VideoAdaptor/Monitor/Modes/Screen/Vendor/ServerFlags/Display/Extensions/DRI/Pointer/Keyboard (v0.3.565)
009 keep: Function List Unbound/NSD server:/remote-control:/stub-zone:/forward-zone:/auth-zone:/view:/python:/dynlib:/dnscrypt:/cachedb:/dnstap:/rpz:/verify:/key:/pattern:/zone:/tls-auth: (v0.3.564)
009 keep: Function List Fail2ban [DEFAULT]/[Definition]/[Init]/[Init?…]/[INCLUDES] (v0.3.562)
009 keep: Function List OpenVPN inline ca/cert/key/dh/pkcs12/extra-certs/crl-verify/tls-auth/tls-crypt/tls-crypt-v2/auth-user-pass/http-proxy-user-pass/auth-gen-token-secret/peer-fingerprint/verify-hash/connection (v0.3.559)
009 keep: Function List strongSwan/Libreswan ipsec.conf config setup/conn/ca (v0.3.558)
009 keep: Function List FreeRADIUS client/home_server/home_server_pool/realm/server/modules/instantiate/listen/policy/files/authorize/authenticate/preacct/accounting/session/post-auth/pre-proxy/post-proxy/recv-coa/send-coa (v0.3.557)
009 keep: Function List ISC dhcpd.conf subnet/subnet6/host/group/shared-network/pool/class/failover peer/on commit|expiry|release (v0.3.554)
009 keep: Function List keepalived.conf global_defs/static_*/vrrp_instance/vrrp_script/vrrp_sync_group/vrrp_track_*/virtual_server (v0.3.553)
009 keep: Function List nftables table/chain/set/map/flowtable/counter/quota/limit/secmark/synproxy/meter/ct helper|timeout|expectation (v0.3.552)
009 keep: Function List BIND/named.conf options/logging/controls/zone/view/acl/key/server/primaries/masters/tls/http/dnssec-policy (v0.3.551)
009 keep: Function List supervisord.conf unix_http_server/inet_http_server/supervisord/supervisorctl/include/program:/group:/eventlistener:/fcgi-program:/rpcinterface: (v0.3.550)
009 keep: Function List Telegraf.conf [[inputs.*]]/[[outputs.*]]/[[processors.*]]/[[aggregators.*]]/[[secretstores.*]]/[agent]/[global_tags] (v0.3.548)
009 keep: Function List Vector.toml sources/transforms/sinks/enrichment_tables/tests/secrets/api (v0.3.547)
009 keep: Function List Fluent Bit SERVICE/INPUT/FILTER/OUTPUT/PARSER/MULTILINE_PARSER/CUSTOM/PROCESSOR/CHECK (v0.3.546)
009 keep: Function List HAProxy global/defaults/traces/frontend/backend/listen/resolvers/cache/peers/mailers/userlist/http-errors/ring/program/fcgi-app (v0.3.545)
009 keep: Function List Apache httpd VirtualHost/Directory/Files/Location/IfModule/If/Else/Proxy/Require*/Limit/Macro (v0.3.544)
009 keep: Function List nginx events/http/server/upstream/location/map/geo/split_clients/limit_except (v0.3.543)
009 keep: Function List sees AWS config [default]/[profile …]/[sso-session …]/[services …]/[plugins] (v0.3.539)
009 keep: Function List sees EditorConfig glob sections [*] / [*.rs] / [*.{js,ts}] (v0.3.536)
009 keep: Function List sees Desktop Entry [Desktop Entry]/[Desktop Action …] (v0.3.534)
009 keep: Function List sees tox.ini [tox]/[testenv]/[testenv:…]/[gh-actions] (v0.3.533)
009 keep: Function List sees Cargo.toml [package]/[lib]/[[bin]]/[workspace]/[profile.*] (v0.3.523)
009 keep: Function List sees Rake task/multitask/file/directory/rule (v0.3.521)
009 keep: Function List sees Premake workspace/project/solution (v0.3.519)
009 keep: Function List sees Apache Ant / MSBuild XML <target name> / <project name> / <macrodef name> / <Target Name> (v0.3.518)
009 keep: Function List sees SCons Program/SharedLibrary/Library/Object/Command/Alias/Install (v0.3.517)
009 keep: Function List sees Arch PKGBUILD / Alpine APKBUILD prepare/build/check/package/pkgver (v0.3.513)
009 keep: Function List sees Debian debian/rules override_dh_*/execute_before_dh_*/execute_after_dh_* (v0.3.512)
009 keep: Function List sees Autoconf/Automake/Libtool/AX_*/PKG_CHECK_MODULES macros (v0.3.511)
009 keep: Function List sees RPM .spec %prep/%build/%install/%package/%define (v0.3.510)
009 keep: Function List sees Ninja rule / build / pool / default (v0.3.506)
009 keep: Function List sees Bazel/Starlark BUILD rules with same-line name= (v0.3.503)
009 keep: Function List sees GN executable/shared_library/source_set/rust_*/target (v0.3.502)
009 keep: Function List sees Meson project/executable/library targets (v0.3.501)
009 keep: Function List sees Dockerfile FROM … AS stage (v0.3.500)
009 keep: Function List sees Vimscript function! / def! / augroup / command (v0.3.499)
009 keep: Function List sees Fortran program / block data / submodule (v0.3.497)
009 keep: Function List sees D unittest / mixin template / static this (v0.3.495)
009 keep: Function List sees Standard ML structure / signature / functor (v0.3.494)
009 keep: Function List sees HLSL cbuffer / tbuffer / Texture2D / RWStructuredBuffer / SamplerState / groupshared (v0.3.486)
009 keep: Function List sees WebAssembly Text (module)/(func)/(type)/(memory)/(table)/(global)/(import)/(export)/(start)/(tag)/(elem)/(data) (v0.3.484)
009 keep: Function List sees Azure Bicep param / resource / output / targetScope / metadata (v0.3.483)
009 keep: Function List sees Odin :: proc / struct / enum / union / bit_set / bit_field / distinct (v0.3.482)
009 keep: Function List sees Idris implementation / parameters (v0.3.476)
009 keep: Function List sees Agda postulate / primitive / variable / infix (v0.3.475)
009 keep: Function List sees Isabelle locale / abbreviation / primrec / typedef (v0.3.474)
009 keep: Function List sees TLA+ MODULE / THEOREM / LEMMA / ASSUME (v0.3.473)
009 keep: Function List sees Lean 4 theorem / structure / inductive / instance (v0.3.471)
009 keep: Function List sees Dafny lemma / predicate / datatype / codatatype (v0.3.469)
009 keep: Function List sees Raku grammar / token / rule / regex / role (v0.3.467)
009 keep: Function List sees WASM WIT world / variant / flags (v0.3.466)
009 keep: Function List sees Smithy operation / structure / resource / intEnum (v0.3.465)
009 keep: Function List sees Prisma model / datasource / generator / view (v0.3.464)
009 keep: Function List sees FlatBuffers table / rpc_service (v0.3.463)
009 keep: Function List sees Sass/SCSS @mixin / @function / @keyframes (v0.3.460)
009 keep: Function List sees Erlang module attrs and clauses (v0.3.459)
009 keep: Function List sees Ruby attr_reader / attr_writer / attr_accessor (v0.3.458)
009 keep: Function List sees Objective-C @implementation / @protocol (v0.3.456)
009 keep: Function List sees Verilog/SystemVerilog task / macromodule / program / primitive / checker (v0.3.454)
009 keep: Function List sees VHDL entity / architecture / component / package / package body (v0.3.453)
009 keep: Function List sees CMake function(name) / macro(name) (v0.3.452)
009 keep: Function List sees Zig const type aliases and test blocks (v0.3.450)
009 keep: Function List sees Objective-C instance and class methods (v0.3.448)
009 keep: Function List sees Terraform / HCL resource, data, variable, output, provider, locals, terraform (v0.3.447)
009 keep: Function List sees GraphQL input / scalar / extend / fragment and named operations (v0.3.446)
009 keep: Function List sees protobuf message / service / rpc (v0.3.445)
009 keep: Function List skips a leading C++ template<…> and lists concept (v0.3.444)
009 keep: Function List skips a leading @Annotation / [Attribute] and lists Java @interface (v0.3.443)
009 keep: Function List skips a leading Rust #[attr] on the same line (v0.3.442)
009 keep: Function List sees F# member / override / default (v0.3.441)
009 keep: Function List sees Scala case class / case object (v0.3.440)
009 keep: Function List sees parenthesized Lisp/Scheme/Clojure defun/defn/define (v0.3.439)
009 keep: Function List sees R name <- function / <<- / name <- \(...) (v0.3.437)
009 keep: Function List sees SQL CREATE TABLE/VIEW/FUNCTION/PROCEDURE/TRIGGER/INDEX/SCHEMA/TYPE (v0.3.436)
009 keep: Function List sees Elixir defdelegate/defmacrop, Clojure defn/defn-/defonce/ns, Haskell data/newtype/instance (v0.3.435)
009 keep: Function List sees Elixir defprotocol/defimpl/defstruct/defguard, Pascal procedure, Fortran subroutine (v0.3.434)
009 keep: Function List sees C# partial/readonly/ref/new/delegate, C typedef, Go/Java package (v0.3.431)
009 keep: Function List sees function-like #define macros (v0.3.428)
009 keep: Function List sees named arrow bindings (const greet = () =>, async params, v0.3.427)
009 keep: Function List sees same-line methods, constructors, function*, export default (v0.3.426)
009 keep: Function List sees C/C++/Java/C# return-type declarations (v0.3.425)
009 keep: Function List sees Swift protocol/extension/actor and Kotlin object/data/suspend (v0.3.424)
009 keep: Function List sees Go/Swift func and Kotlin fun (v0.3.423)
009 keep: Function List sees Rust macro_rules, macro, and extern ABI fn (v0.3.422)
009 keep: Function List sees Rust pub(crate) / unsafe / const declarations (v0.3.421)
009 keep: syntax language cycle (shortcut_language, Alt+= default, Shift=previous, v0.3.420)
009 keep: remappable Command Line Arguments (shortcut_cmdline, Shift+F1 default, v0.3.419)
009 keep: remappable Releases / Changelog (shortcut_releases, Cmd+Alt+] default, Shift=Changelog, v0.3.418)
009 keep: remappable Discussions / Wiki (shortcut_discussions, Cmd+Alt+[ default, Shift=Wiki, v0.3.417)
009 keep: remappable npp-rust on GitHub / Issues (shortcut_github, Alt+; default, Shift=Issues, v0.3.416)
009 keep: remappable Open npp-rust Logs (shortcut_open_logs, Alt+F2 default, v0.3.415)
009 keep: remappable Debug Info (shortcut_debug_info, Cmd+Alt+Shift+Space default, v0.3.414)
009 keep: remappable View current file in IE (shortcut_view_in_ie, Alt+Shift+9 default, v0.3.413)
009 keep: remappable View current file in Edge (shortcut_view_in_edge, Alt+Shift+. default, v0.3.412)
009 keep: remappable View current file in Chrome (shortcut_view_in_chrome, Alt+Shift+B default, v0.3.411)
009 keep: remappable View current file in Firefox (shortcut_view_in_browser, Cmd+Shift+Y default, v0.3.410)
009 keep: remappable Paste HTML / Paste RTF (shortcut_paste_html, Cmd+Alt+- default, Shift=RTF, v0.3.409)
009 keep: remappable Paste Binary (shortcut_paste_binary, Cmd+Shift+V default, v0.3.408)
009 keep: remappable Cut Binary (shortcut_cut_binary, Cmd+Alt+Shift+X default, v0.3.406)
009 keep: remappable Copy Binary (shortcut_copy_binary, Cmd+Shift+X default, v0.3.405)
009 keep: Copy/Cut/Paste Binary uses hex of the selection UTF-8 bytes (v0.3.404)
009 keep: remappable system read-only attribute (shortcut_system_readonly, Cmd+Alt+Shift+F default, v0.3.403)
009 keep: remappable SHA-512 generate from file (shortcut_sha512_file, Cmd+Shift+F11 default, v0.3.402)
009 keep: remappable SHA-1 generate from file (shortcut_sha1_file, Alt+F12 default, v0.3.401)
009 keep: remappable SHA-256 generate from file (shortcut_sha256_file, Alt+Shift+H default, v0.3.400)
009 keep: remappable MD5 generate from file (shortcut_hash_file, Cmd+Shift+H default, v0.3.399)
009 keep: Clipboard History cycles the last 10 session copies into the paste clipboard (v0.3.398)
009 keep: Find Mark Style next/prev/copy uses style 1 (Find-bar Mark), not bookmarks (v0.3.397)
009 keep: remappable Multi-select All match case whole word (shortcut_multi_select_all_case_word, Alt+Shift+A default, v0.3.396)
009 keep: remappable Multi-select All match case (shortcut_multi_select_all_case, Cmd+Shift+A default, v0.3.395)
009 keep: remappable Multi-select Next match case whole word (shortcut_multi_select_next_case_word, Cmd+Shift+N default, v0.3.394)
009 keep: remappable Multi-select Next whole word (shortcut_multi_select_next_word, Cmd+Shift+W default, v0.3.393)
009 keep: remappable Multi-select Next match case (shortcut_multi_select_next_case, Alt+Shift+N default, v0.3.392)
009 keep: remappable Block Comment / Block Uncomment (shortcut_stream_comment, Alt+, default; Shift=uncomment, v0.3.391)
009 keep: remappable Single Line Comment / Uncomment (shortcut_line_comment, Alt+/ default; Shift=uncomment, v0.3.390)
009 keep: remappable Run a Macro Multiple Times (shortcut_macro_repeat, Cmd+Alt+Shift+K default, v0.3.389)
009 keep: remappable Save Current Recorded Macro (shortcut_macro_save, Cmd+Alt+Shift+S default, v0.3.388)
009 keep: remappable Start/Stop Recording (shortcut_macro_record, Cmd+Alt+Shift+R default, toggle, v0.3.387)
009 keep: remappable Macro Playback (shortcut_macro_playback, Cmd+Shift+P default, v0.3.386)
009 keep: remappable Date Time customized (shortcut_insert_datetime_custom, Cmd+Alt+= default, v0.3.385)
009 keep: remappable 1st–9th Tab (shortcut_goto_tab, Cmd+Shift+1…9 default, v0.3.384)
009 keep: remappable Move to New Instance / Open in New Instance (shortcut_new_instance, F9 default; Shift=keep tab, v0.3.383)
009 keep: remappable Undo the latest added multi-select (shortcut_multi_select_undo, Cmd+Alt+Backspace default, v0.3.382)
009 keep: remappable Function Parameters Next/Previous (shortcut_calltip_nav, Alt+Down default; opposite Up=prev, v0.3.381)
009 keep: remappable Move to Recycle Bin (shortcut_delete_file, Cmd+Shift+Delete default, v0.3.380)
009 keep: remappable Clipboard History (shortcut_clipboard_history, Cmd+Shift+C default, v0.3.376)

009 keep: remappable Tab colour 5 / clear (shortcut_tab_colour_5, Cmd+; default; Shift=clear, v0.3.375)
009 keep: remappable Tab colour 4 / clear (shortcut_tab_colour_4, Cmd+M default; Shift=clear, v0.3.374)
009 keep: remappable Tab colour 3 / clear (shortcut_tab_colour_3, Cmd+K default; Shift=clear, v0.3.373)
009 keep: remappable Tab colour 2 / clear (shortcut_tab_colour_2, Cmd+F4 default; Shift=clear, v0.3.372)
009 keep: remappable Tab colour 1 / clear (shortcut_tab_colour, Cmd+F5 default; Shift=clear, v0.3.371)
009 keep: remappable Shortcut Mapper (shortcut_mapper, Alt+Shift+K default, v0.3.370)
009 keep: remappable SHA-512 generate / clipboard (shortcut_sha512, Cmd+F6 default; Shift=clipboard, v0.3.369)
009 keep: remappable SHA-1 generate / clipboard (shortcut_sha1, Cmd+F12 default; Shift=clipboard, v0.3.368)
009 keep: remappable SHA-256 generate / clipboard (shortcut_sha256, Alt+F11 default; Shift=clipboard, v0.3.367)
009 keep: remappable MD5 generate / clipboard (shortcut_md5, Alt+F10 default; Shift=clipboard, v0.3.366)
009 keep: remappable ANSI (shortcut_encoding_ansi, Cmd+F11 default, v0.3.365)
009 keep: remappable UTF-16 LE / UTF-16 BE (shortcut_encoding_utf16, Cmd+F10 default; Shift=BE, v0.3.364)
009 keep: remappable UTF-8 / UTF-8-BOM (shortcut_encoding_utf8, Cmd+F9 default; Shift=BOM, v0.3.363)
009 keep: remappable Sort tabs by modified time (shortcut_sort_tabs_modified, Cmd+Alt+; default; Shift=desc, v0.3.362)
009 keep: remappable Sort tabs by size (shortcut_sort_tabs_size, Cmd+Alt+/ default; Shift=desc, v0.3.361)
009 keep: remappable Sort tabs by type (shortcut_sort_tabs_type, Cmd+Alt+. default; Shift=desc, v0.3.360)
009 keep: remappable Sort tabs by path (shortcut_sort_tabs_path, Cmd+Alt+, default; Shift=desc, v0.3.359)
009 keep: remappable Sort tabs by name (shortcut_sort_tabs, Cmd+Alt+F1 default; Shift=desc, v0.3.358)
009 keep: remappable Load Session / Save Session (shortcut_load_session, Cmd+F1 default; Shift=save, v0.3.357)
009 keep: remappable Exit (shortcut_exit, Cmd+Q default, v0.3.356)
009 keep: remappable Preferences / Style Configurator (shortcut_preferences, Cmd+, default; Shift=style, v0.3.355)
009 keep: remappable Summary / Document List (shortcut_summary, Alt+F1 default; Shift=doc list, v0.3.354)
009 keep: remappable Read-only for all documents (shortcut_readonly_all, F10 default; Shift=clear, v0.3.353)
009 keep: remappable Show Wrap Symbol (shortcut_show_wrap_symbol, Alt+Shift+G default, v0.3.352)
009 keep: Show Wrap Symbol toggles the soft-wrap mark (show_wrap_symbol, default on, v0.3.351)
009 keep: remappable Hide Lines (shortcut_hide_lines, Alt+Shift+L default, v0.3.350)
009 keep: remappable Search on Internet (shortcut_search_internet, Alt+U default; Shift=Change Search Engine, v0.3.348)
009 keep: remappable Redact Selection (shortcut_redact, Alt+J; Shift=●, v0.3.346)
009 keep: remappable Random Case (shortcut_random_case, Cmd+Alt+0 default, v0.3.345)
009 keep: remappable Sentence Case Force/Blend (shortcut_sentence_case, Cmd+Alt+8 default; Shift=blend, v0.3.343)
009 keep: remappable EOL to Space (shortcut_eol_to_space, Cmd+Alt+F11 default; Shift=Trim All, v0.3.339)
009 keep: remappable Sort Lines by Length (shortcut_sort_lines_length, Cmd+Alt+F10 default; Shift=desc, v0.3.338)
009 keep: remappable Sort Lines Decimal Comma (shortcut_sort_lines_decimal_comma, Cmd+Alt+F9 default; Shift=desc, v0.3.337)
009 keep: remappable Sort Lines Decimal Dot (shortcut_sort_lines_decimal_dot, Cmd+Alt+F8 default; Shift=desc, v0.3.336)
009 keep: remappable Sort Lines Lex Ignoring Case (shortcut_sort_lines_case_insensitive, Cmd+Alt+F6 default; Shift=desc, v0.3.335)
009 keep: remappable Sort Lines Integer (shortcut_sort_lines_integer, Cmd+Alt+F5 default; Shift=desc, v0.3.334)
009 keep: Find Next/Previous (Volatile) selection query + remappable shortcut_find_volatile (Cmd+Alt+F3 default; Shift=prev, v0.3.333)
009 keep: remappable Find characters in range (shortcut_find_char_range, Alt+F9 default; Shift=prev, v0.3.332)
009 keep: remappable Find Purge (shortcut_find_purge, Alt+Shift+F8 default; Shift=substrings, v0.3.331)
009 keep: remappable Find Cut (shortcut_find_cut, Alt+Shift+F6 default; Shift=lines, v0.3.330)
009 keep: remappable Find Mark (shortcut_find_mark, Alt+Shift+F3 default, v0.3.327)
009 keep: remappable Mark current line Style (shortcut_mark_one_style, Alt+Shift+1 default, digit-family 1…5, v0.3.326)
009 keep: remappable Copy Styled Text (shortcut_style_to_clip, Cmd+Alt+Shift+1 default, digit-family 1…5, 0=all, v0.3.325)
009 keep: remappable Clear Change History (shortcut_clear_change_history, Cmd+Shift+F8 default, v0.3.324)
009 keep: remappable Incremental Search (shortcut_find_incremental, Alt+Shift+I default, v0.3.323)
009 keep: remappable Remove Unmarked Lines (shortcut_remove_unmarked, Cmd+Alt+Z default, v0.3.322)
009 keep: Find-bar Incremental Search (Inc; as-you-type jump, v0.3.321)
009 keep: Find Results / Find in Files Shift+Enter = Previous Search Result (v0.3.320)
009 keep: remappable Next/Previous Style marker (shortcut_next_style, Cmd+Alt+1 default, digit-family 1…5, Shift=prev, v0.3.319)
009 keep: Replace-bar Enter = Replace / Shift+Enter = Replace All (v0.3.318)
009 keep: Find-bar Shift+Enter = Find Previous (v0.3.317)
009 keep: Find Results / Find in Files dual-view secondary double-click hit jump (v0.3.316)
009 keep: remappable Find All / Find All in opened (shortcut_find_all, Cmd+Alt+F4 default, Shift=opened, v0.3.314)
009 keep: Find Results / Find in Files double-click hit jump (v0.3.313)
009 keep: Next/Prev Search Result walks Find in Files hits (open from workspace, v0.3.312)
009 keep: Find-bar Shift+All / Find All in opened docs (Case/Word/Re; Sel ignored, v0.3.310)
009 keep: Find-bar All / Search Results Window (current-doc Find Results, Case/Word/Re/Sel, v0.3.309)
009 keep: Find-bar Shift+Cut matching lines (Case/Word/Re/Sel, v0.3.307)
009 keep: Find-bar Purge matching lines (Case/Word/Re/Sel, v0.3.305)
009 keep: Find-bar Bookmark all matching lines (Case/Word/Re/Sel, v0.3.302)
009 keep: remappable Space to TAB (All) (shortcut_space_to_tab_all, Cmd+Alt+X default, v0.3.300)
009 keep: remappable Paste/Remove Bookmarked Lines (shortcut_paste_marked_lines, Cmd+Alt+Q default, Shift=remove, v0.3.299)
009 keep: remappable First/Last Tab (shortcut_first_tab, Cmd+Alt+Home default, opposite End=last, v0.3.296)
009 keep: remappable Next/Previous Search Result (shortcut_next_found, F4 default, Shift=prev, v0.3.295)
009 keep: remappable Path autocomplete (shortcut_autocomplete_path, Cmd+Alt+Space default, v0.3.294)
009 keep: remappable Autocomplete / Function Call Tip (shortcut_autocomplete, Cmd+Space default, Shift=call tip, v0.3.293)
009 keep: remappable Fold Level 1…8 (shortcut_fold_level, Alt+1 default, digit-family 1…8, Shift=unfold, v0.3.292)
009 keep: remappable rect column select (shortcut_rect_select, Alt+Shift+Left default, arrow-family any arrow, v0.3.291)
009 keep: remappable Zoom Sync (shortcut_zoom_sync, Alt+Shift+Z default, v0.3.290)
009 keep: remappable Inverse Bookmarks (shortcut_inverse_bookmarks, Alt+Shift+F2 default, v0.3.289)
009 keep: remappable Mark Style 5 / Clear Style 5 (shortcut_mark_style_5, Cmd+5 default, Shift=clear, v0.3.287)
009 keep: remappable Mark Style 4 / Clear Style 4 (shortcut_mark_style_4, Cmd+4 default, Shift=clear, v0.3.286)
009 keep: remappable Mark Style 3 / Clear Style 3 (shortcut_mark_style_3, Cmd+3 default, Shift=clear, v0.3.285)
009 keep: remappable Mark Style 2 / Clear Style 2 (shortcut_mark_style_2, Cmd+2 default, Shift=clear, v0.3.284)
009 keep: remappable Mark Style 1 / Clear Style 1 (shortcut_mark_style, Cmd+1 default, Shift=clear, v0.3.283)
009 keep: remappable Move Tab Forward/Backward (shortcut_move_tab, Cmd+Shift+PageDown default, opposite PageUp=back, v0.3.281)
009 keep: remappable Function List / Character Panel (shortcut_function_list, Cmd+E default, Shift=char panel, toggle both, v0.3.280)
009 keep: remappable Save a Copy As / Rename (shortcut_save_copy, Cmd+Alt+Y default, Shift=rename, v0.3.279)
009 keep: remappable Close All to the Left / Right (shortcut_close_to_side, Cmd+Alt+I default, Shift=right, v0.3.278)
009 keep: remappable Close All but Pinned / Close All Unchanged (shortcut_close_but_pinned, Cmd+Alt+H default, Shift=unchanged, v0.3.277)
009 keep: remappable Open Folder as Workspace / Containing Folder (shortcut_open_workspace, Cmd+Alt+G default, Shift=containing, v0.3.276)
009 keep: remappable Open Containing Folder / Shell (shortcut_open_folder, Cmd+Alt+L default, Shift=shell, v0.3.275)
009 keep: remappable Open Selected Filename / Containing Folder (shortcut_open_selected, Cmd+Alt+J default, Shift=folder, v0.3.274)
009 keep: remappable Copy All Paths / All Filenames (shortcut_copy_all_paths, Cmd+Alt+M default, Shift=names, v0.3.273)
009 keep: remappable Copy Current Directory Path (shortcut_copy_dir, Cmd+Alt+K default, v0.3.272)
009 keep: remappable Copy Full Path / File Name (shortcut_copy_path, Cmd+Alt+P default, Shift=name, v0.3.271)
009 keep: remappable Date Time short/long (shortcut_insert_datetime, F5 default, Shift=long, v0.3.270)
009 keep: remappable Remove Duplicate Lines (shortcut_remove_dup_lines, Cmd+Alt+D default, Shift=any, v0.3.269)
# Session todo

- [x] Close #8 (themes / history / UTF-16)
- [x] File gap issues #9–#14
- [x] Loop: #9 hotkeys (coder → TEST)
- [x] Loop: #10 DnD coder → TEST
- [x] Loop: #11 column TEST → DONE (open for handoff)
- [x] Loop: #12 Find in Files TEST → DONE (open for handoff)
- [x] Loop: #14 TEST / handoff
- [x] Loop: #15 About window TEST / handoff
- [x] Loop: #16 scrollbar TEST → DONE (closed)
- [x] Loop: #17 submenu size TEST → DONE (closed)
- [x] Loop: #18 Ollama helper TEST → DONE (closed)
- [x] Loop: #19 Ask Ollama background TEST → DONE (closed)
- [x] Loop: #20 next-gaps.md refresh TEST → DONE (closed)
- [x] Loop: #21 Ollama Chat TEST → DONE (open for close)
- [ ] Run `./scripts/ci-local.sh` before every push
- [ ] Skim `logs/panic.log` if the app crashed
- 009 keep: Find wrap toggle (v0.3.14); regex still open
- 009 keep: Find regex (linear-time Re toggle, v0.3.15)
- 009 keep: Find Sel auto-arm on multi-line selection (v0.3.16)
- 009 keep: BOM-less UTF-16 LE/BE detect on open (v0.3.17)
- 009 keep: multi-caret typing coalesce (v0.3.18)
- 009 keep: remappable word-wrap shortcut (v0.3.19)
- 009 keep: compare next/prev difference (F7, v0.3.20)
- 009 keep: fold margin click parity with Fold Current (v0.3.21)
- 009 keep: project panel refresh + reveal in file manager (v0.3.22)
- 009 keep: compare first-hunk jump + ordinal status (v0.3.23)
- 009 keep: compare ignore-case preference (v0.3.24)
- 009 keep: compare dual-pane hunk align + identical status (v0.3.25)
- 009 keep: compare first/last difference (⌘/Ctrl+F7, v0.3.26)
- 009 keep: compare survives closing a non-pair tab (v0.3.27)
009 keep: compare Swap Compare Sides (v0.3.28)
009 keep: compare View ignore toggles + start dual-pane park (v0.3.29)
009 keep: compare click-hunk sync + L|R nav status (v0.3.30)
009 keep: compare keyboard caret hunk sync (v0.3.31)
009 keep: compare next/prev wrap status (v0.3.32)
009 keep: compare nav selects hunk on focused pane (v0.3.33)
009 keep: recent_max honored on recent.txt load (v0.3.34)
009 keep: View whitespace/EOL/NPC/indent-guide persist (v0.3.35)
009 keep: compare both-pane hunk select on nav/click/start (v0.3.36)
009 keep: settings.json unknown keys survive save (v0.3.37)
009 keep: compare Copy Compare Diff (unified, v0.3.38)
009 keep: compare Copy Compare Hunk (v0.3.39)
009 keep: compare Apply Hunk From Other View (v0.3.40)
009 keep: compare Apply All Hunks From Other View (v0.3.42)
009 keep: Find in Files honors Re (linear-time regex, v0.3.43)
009 keep: compare Open Compare Diff tab (v0.3.44)
009 keep: regex Replace $n / \1 capture expansion (v0.3.45)
009 keep: compare Apply Hunk advances to next remaining (v0.3.46)
009 keep: compare Ignore Blank Lines (v0.3.47)
009 keep: compare Open Compare Hunk tab (v0.3.48)
009 keep: compare Apply Hunk To Other View (v0.3.49)
009 keep: compare Apply All Hunks To Other View (v0.3.50)
009 keep: compare Compare to Saved disk snapshot (v0.3.51)
009 keep: compare Clear closes saved snapshot tab (v0.3.52)
009 keep: compare word-aware replace-hunk wash (v0.3.53)
009 keep: compare Hide Unchanged Lines (v0.3.54)
009 keep: compare hide-equal ±3 context (v0.3.55)
009 keep: compare hide-equal ···N gap cues + hidden counts (v0.3.56)
009 keep: compare click ···N cue expands Equal run (v0.3.57)
009 keep: compare ···N expand both panes (aligned Equal) (v0.3.58)
009 keep: compare Expand All Unchanged Lines (v0.3.59)
009 keep: compare Collapse All Unchanged Lines (v0.3.60)
009 keep: compare Bookmark Compare Differences (v0.3.61)
009 keep: compare Clear Compare Difference Bookmarks (v0.3.62)
009 keep: compare Equal-line partner park (v0.3.63)
009 keep: compare Expand Unchanged at Caret (v0.3.64)
009 keep: compare Collapse Unchanged at Caret (v0.3.65)
009 keep: session fold state (@folds headers) (v0.3.66)
009 keep: compare Next/Previous Hidden Equal (v0.3.67)

009 keep: compare First/Last Hidden Equal (v0.3.68)
009 keep: compare hide-equal context pref (0–10) (v0.3.69)
009 keep: compare Open Compare Summary (v0.3.70)

009 keep: compare Copy Compare Summary (v0.3.71)
009 keep: compare summary first-line previews (v0.3.72)
009 keep: compare summary kind tallies (v0.3.73)
009 keep: compare summary ignore flags (v0.3.74)

009 keep: compare summary multi-line previews (+N more) (v0.3.75)

009 keep: compare summary equal-line match percent (v0.3.76)

009 keep: compare live status · N% equal (v0.3.77)

009 keep: compare live status hunk kind tallies (v0.3.78)

009 keep: compare nav/click current hunk kind (v0.3.79)

009 keep: compare nav/click current hunk −/+ counts (v0.3.80)

009 keep: compare apply hunk status kind/−/+ (v0.3.81)

009 keep: compare apply all hunks status kind/−/+ (v0.3.82)

009 keep: compare copy/open hunk status kind/−/+ (v0.3.83)

009 keep: compare copy/open summary status kind tallies (v0.3.84)

009 keep: compare copy/open unified diff status kind tallies (v0.3.85)

009 keep: compare start parks first hunk · at L|R (v0.3.86)

009 keep: compare ignore toggle re-diff parks first hunk · at L|R (v0.3.87)

009 keep: compare swap sides re-diff parks first hunk · at L|R (v0.3.88)

009 keep: compare Clear status −/+/kind tallies (v0.3.89)

009 keep: compare bookmark/clear difference bookmarks status −/+/kind tallies (v0.3.90)

009 keep: compare expand/collapse all unchanged status −/+/kind tallies (v0.3.91)

009 keep: compare expand/collapse ···N at caret status −/+/kind tallies (v0.3.92)

009 keep: compare next/prev/first/last hidden equal status −/+/kind tallies (v0.3.93)

009 keep: compare apply hunk From/To hotkeys (v0.3.94)

009 keep: compare apply all hunks From/To hotkeys (v0.3.95)

009 keep: compare next/prev/first/last hidden equal hotkeys (v0.3.96)

009 keep: compare hide-equal toggle parks first hunk · at L|R (v0.3.97)

009 keep: compare hide-equal context change parks first hunk · at L|R (v0.3.98)

009 keep: compare edit re-diff parks partner + · at L|R (v0.3.99)

009 keep: compare equal partner park reveals hide-equal + · equal L|R (v0.3.100)

009 keep: compare equal park status −/+/kind tallies (v0.3.101)

009 keep: compare equal park · N% equal (v0.3.102)

009 keep: compare equal park ignore + hide-equal bits (v0.3.103)

009 keep: remappable Find next shortcut (shortcut_find_next, v0.3.104)

009 keep: remappable next bookmark shortcut (shortcut_next_bookmark, v0.3.105)

009 keep: remappable next difference shortcut (shortcut_next_diff, v0.3.106)

009 keep: remappable go to line shortcut (shortcut_goto_line, v0.3.107)

009 keep: remappable duplicate line shortcut (shortcut_duplicate_line, v0.3.108)

009 keep: remappable delete line shortcut (shortcut_delete_line, v0.3.109)

009 keep: remappable indent shortcut (shortcut_indent, v0.3.110)

009 keep: remappable outdent shortcut (shortcut_outdent, v0.3.111)

009 keep: remappable toggle bookmark shortcut (shortcut_toggle_bookmark, v0.3.112)

009 keep: remappable format document shortcut (shortcut_format_document, v0.3.113)

009 keep: remappable close tab shortcut (shortcut_close_tab, v0.3.114)

009 keep: remappable save shortcut (shortcut_save, v0.3.115)

009 keep: remappable find shortcut (shortcut_find, v0.3.116)

009 keep: remappable replace shortcut (shortcut_replace, v0.3.117)

009 keep: remappable Save As shortcut (shortcut_save_as, v0.3.118)

009 keep: remappable New shortcut (shortcut_new, v0.3.119)

009 keep: remappable Open shortcut (shortcut_open, v0.3.120)

009 keep: remappable Select all shortcut (shortcut_select_all, v0.3.121)

009 keep: remappable Undo shortcut (shortcut_undo, v0.3.122)

009 keep: remappable Redo shortcut (shortcut_redo, v0.3.123)

009 keep: compare soft-truncate oversize pairs (first 3000 lines + status, v0.3.124)

009 keep: remappable Zoom in shortcut (shortcut_zoom_in, v0.3.125)

009 keep: remappable Zoom out shortcut (shortcut_zoom_out, v0.3.126)

009 keep: remappable Zoom restore shortcut (shortcut_zoom_restore, v0.3.127)

009 keep: remappable Toggle log tail shortcut (shortcut_toggle_log_tail, v0.3.128)

009 keep: remappable Find next (global) shortcut (shortcut_find_next_global, v0.3.129)

009 keep: remappable Replace (alternate) shortcut (shortcut_replace_alt, v0.3.130)

009 keep: remappable Next hidden equal shortcut (shortcut_next_hidden_equal, v0.3.131)

009 keep: remappable Apply compare hunk shortcut (shortcut_apply_compare_hunk, v0.3.132)

009 keep: remappable Hide-equal context shortcut (shortcut_hide_equal_context, v0.3.133)

009 keep: remappable Reload shortcut (shortcut_reload, v0.3.134)

009 keep: remappable First difference shortcut (shortcut_first_diff, v0.3.135)

009 keep: remappable First hidden equal shortcut (shortcut_first_hidden_equal, v0.3.136)

009 keep: remappable Hide unchanged lines shortcut (shortcut_hide_equal, v0.3.137)

009 keep: remappable Compare start/clear shortcut (shortcut_compare, Alt+D default, v0.3.138)

009 keep: remappable Close Find/Replace shortcut (shortcut_close_find, v0.3.140)

009 keep: remappable Swap Compare sides shortcut (shortcut_swap_compare, Alt+S default, v0.3.141)

009 keep: remappable Word jump shortcut (shortcut_word_jump, Alt+Left default, v0.3.142)

009 keep: remappable Compare to Saved shortcut (shortcut_compare_to_saved, Alt+Shift+S default, v0.3.143)

009 keep: remappable Next / prev tab shortcut (shortcut_next_tab, Cmd+Tab default, v0.3.144)

009 keep: remappable Save All shortcut (shortcut_save_all, Cmd+Alt+S default, v0.3.145)

009 keep: remappable Print shortcut (shortcut_print, Cmd+P default, v0.3.146)

009 keep: remappable Shift+Tab outdent (shortcut_shift_tab_outdent, Shift+Tab default, v0.3.147)

009 keep: remappable Fold / Unfold all (shortcut_fold_all, Alt+0 default, v0.3.148)

009 keep: remappable Fold / Unfold current (shortcut_fold_current, Alt+F default, v0.3.149)

009 keep: remappable Matching brace (shortcut_matching_brace, Cmd+B default, v0.3.150)

009 keep: remappable Move line (shortcut_move_line, Cmd+Shift+Up default, v0.3.151)

009 keep: remappable Toggle comment (shortcut_toggle_comment, Cmd+/ default, v0.3.152)

009 keep: remappable Copy / Open Compare Diff (shortcut_copy_compare_diff, Alt+C default, v0.3.153)
009 keep: remappable Copy / Open Compare Summary (shortcut_copy_compare_summary, Alt+Y default, v0.3.154)
009 keep: remappable Copy / Open Compare Hunk (shortcut_copy_compare_hunk, Alt+K default, v0.3.155)

009 keep: remappable Tab indent (shortcut_tab_indent, Tab default, v0.3.156)

009 keep: remappable Compare ignore whitespace / ignore case (shortcut_compare_ignore_ws, Alt+W default, v0.3.157)

009 keep: remappable Compare ignore blank lines (shortcut_compare_ignore_blank, Alt+B default, v0.3.158)

009 keep: remappable Expand / Collapse All Unchanged (shortcut_expand_all_unchanged, Alt+E default, v0.3.159)

009 keep: remappable Expand / Collapse Unchanged at Caret (shortcut_expand_unchanged_at_caret, Alt+X default, v0.3.160)

009 keep: remappable Bookmark / Clear Compare Difference Bookmarks (shortcut_bookmark_compare_diffs, Alt+M default, v0.3.162)

009 keep: scrollable tall menus + menu shortcut labels (v0.3.164)

009 keep: Document Map compare colours (focused pane, v0.3.166)

009 keep: Document Map click parks compare hunk / Equal partner (v0.3.167)

009 keep: Document Map hunk-start ticks + drag sync scroll (v0.3.168)
009 keep: Document Map hide-equal ···N gap bands + click park (v0.3.169)
009 keep: Document Map bookmark ticks + click park (v0.3.170)
009 keep: Document Map change-history ticks + click park (v0.3.171)
009 keep: Document Map Find match ticks + click select (v0.3.172)

009 keep: Document Map fold-header ticks + click toggle (v0.3.173)
009 keep: Document Map selection wash + caret / multi-caret ticks (v0.3.174)

009 keep: remappable Document Map toggle (shortcut_document_map, Cmd+Shift+D default, v0.3.175)

009 keep: remappable Find in Files (shortcut_find_in_files, Cmd+Alt+F default, v0.3.176)
009 keep: Document Map viewport band sized to editor page (v0.3.177)
009 keep: remappable go to document start/end (shortcut_goto_document_start, Cmd+Up default, v0.3.178)
009 keep: Focus on Another View toggles pane focus (F6 remappable; Shift swaps; v0.3.179)
009 keep: remappable Next / Previous Change History (shortcut_next_changed, F8 default, v0.3.180)
009 keep: remappable lowercase / UPPERCASE (shortcut_lowercase, Cmd+U default, v0.3.181)
009 keep: remappable Join Lines / Split Lines (shortcut_join_lines, Cmd+J default, v0.3.182)

009 keep: remappable Invert Case / Proper Case (shortcut_invert_case, Cmd+Alt+U default, v0.3.183)

009 keep: remappable blank line below / above (shortcut_blank_line, Cmd+Enter default, v0.3.184)

009 keep: remappable Clear all bookmarks (shortcut_clear_bookmarks, Cmd+Shift+F2 default, v0.3.185)

009 keep: remappable Copy (shortcut_copy, Cmd+C default, v0.3.186)

009 keep: remappable Cut (shortcut_cut, Cmd+X default, v0.3.187)

009 keep: remappable Paste (shortcut_paste, Cmd+V default, v0.3.188)

009 keep: remappable Delete forward (shortcut_delete_forward, Delete default, v0.3.189)

009 keep: remappable Backspace / delete backward (shortcut_delete_backward, Backspace default, v0.3.190)

009 keep: remappable line start / end (shortcut_line_home, Home default, v0.3.191)

009 keep: remappable Page up / down (shortcut_page_up, PageUp default, v0.3.192)

009 keep: remappable Character left / right (shortcut_char_left, Left default, v0.3.193)

009 keep: remappable line up / down (shortcut_line_up, Up default, v0.3.194)
009 keep: Shift+Up/Down and Shift+PageUp/PageDown selection extend (v0.3.195)

009 keep: remappable Document Home / End aliases (shortcut_document_home, Cmd+Home default, v0.3.196)

009 keep: Clear Compare restores pre-start sync H/V scroll (v0.3.197)

009 keep: Preferences Compare ignore/hide toggles match View live park (v0.3.198)

009 keep: RTL/LTR text_rtl persists (Edit + Preferences, v0.3.199)

009 keep: remappable Enter / newline (shortcut_enter, Enter default, v0.3.200)

009 keep: remappable Multi-select Next / Skip (shortcut_multi_select_next, Cmd+Alt+N default, v0.3.201)

009 keep: remappable Fullscreen (shortcut_fullscreen, F11 default, v0.3.202)

009 keep: remappable Always on Top toggle (shortcut_always_on_top, Alt+T default, v0.3.203)

009 keep: dual-view cross-pane selection drag move/copy (v0.3.204)

009 keep: drop selection into Find field (v0.3.205)

009 keep: drop selection into Replace field (v0.3.206)
009 keep: remappable Show All Characters (v0.3.207)

009 keep: remappable Indent Guide (v0.3.208)

009 keep: remappable Project Panel toggle (shortcut_project_panel, Alt+P default, v0.3.209)

009 keep: remappable Pin Tab (shortcut_pin_tab, Alt+Shift+P default, v0.3.210)

009 keep: remappable Show White Space / Show EOL (shortcut_show_whitespace, Alt+V default, v0.3.211)

009 keep: remappable Sync Vertical / Horizontal Scroll (shortcut_sync_scroll, Alt+Q default, v0.3.212)

009 keep: remappable Show NPC (shortcut_show_npc, Alt+N default, v0.3.213)

009 keep: remappable Post-It (shortcut_postit, F12 default, v0.3.214)
---
009 keep: dual-view other pane syntax colours (v0.3.215)
009 keep: remappable Move / Clone to Other View (shortcut_move_other_view, Alt+O; Shift clones, v0.3.216)

009 keep: remappable RTL / LTR (shortcut_rtl, Alt+R; Shift = LTR, v0.3.217)

009 keep: remappable Show Line Numbers (shortcut_show_line_numbers, Alt+L default, v0.3.218)

009 keep: session restore caret + active tab (@caret / @active, v0.3.219)

009 keep: remappable Show Fold Margin (shortcut_show_fold_margin, Alt+G default, v0.3.220)

009 keep: session restore bookmarks (@bookmarks, v0.3.221)

009 keep: session restore pin (@pin, v0.3.222)

009 keep: session restore language (@lang, v0.3.223)

009 keep: session restore encoding (@encoding, v0.3.224)

009 keep: session restore read-only (@readonly, v0.3.225)

009 keep: session restore scroll (@scroll active tab, v0.3.226)
009 keep: per-tab session @scroll + tab-switch viewport (v0.3.227)

009 keep: session restore tab colour (@colour / @color 1..=5, v0.3.228)

009 keep: session restore log-tail (@tail / @logtail, v0.3.229)

009 keep: session restore selection (@sel, v0.3.230)

009 keep: session restore style marks (@style1..@style5, v0.3.231)

009 keep: session restore dual-view other pane (@other / @otherview, v0.3.232)

009 keep: session restore dual-view other-pane focus (@focus, v0.3.233)

009 keep: session restore sync-scroll (@syncv / @synch, v0.3.234)

009 keep: Always on Top persists (`always_on_top`, v0.3.235)

009 keep: Document Map open persists (`show_document_map`, v0.3.236)

009 keep: session restore Compare on (@compare / @cmp, v0.3.237)

009 keep: Project panel open persists (`show_project_panel`, v0.3.238)

009 keep: session restore marked Compare partner (@partner / @cmppartner, v0.3.239)

009 keep: Zoom Sync persists + sync toggles flip live app state (v0.3.240)

009 keep: Post-It distraction-free hide menu+status + persist (post_it, v0.3.241)

009 keep: Alt+drag rect virtual space pad-on-type/copy (v0.3.244)

009 keep: Alt+drag rect click/drag past EOL virtual carets (v0.3.245)

009 keep: plain free caret past EOL click/Right + pad-on-type (v0.3.246)

009 keep: stream select into virtual space Shift+drag/Right (v0.3.247)

009 keep: multi-caret plain arrow move (v0.3.248)

009 keep: Ctrl/Cmd+click add/toggle multi-carets (v0.3.249)

009 keep: Shift+extend stream multi-carets (v0.3.250)

009 keep: Rect Shift+extend Left/Right/Up/Down (v0.3.251)

009 keep: multi-caret word-jump move + Shift-extend (v0.3.252)

009 keep: multi-caret line Home/End move + Shift-extend (v0.3.253)

009 keep: multi-caret Page Up/Down move + Shift-extend (v0.3.254)

009 keep: Escape clears multi-carets / rect when Find closed (v0.3.255)

009 keep: remappable Close All / Close All but Current (v0.3.256)

009 keep: remappable Trim Trailing / Trim Both (v0.3.257)

009 keep: multi-caret / rect paste line-split (v0.3.258)

009 keep: remappable Multi-select All / whole-word (v0.3.259)
009 keep: Begin/End Select column mode rect + remappable shortcut (v0.3.260)

009 keep: Begin/End Select column mode virtual cols past EOL (v0.3.261)

009 keep: Rect Shift+Home/End extend (v0.3.262)

009 keep: Alt+Shift+arrows start/extend rect column select (v0.3.263)

009 keep: Document Home/End move / Shift-extend multi-carets / rect (v0.3.264)

009 keep: remappable Toggle Read-Only (v0.3.265)

009 keep: remappable Column Editor (v0.3.266)

009 keep: remappable Sort Lines Lexicographic Asc/Desc (v0.3.267)

009 keep: remappable Remove Empty Lines / Containing Blank (v0.3.268)

009 keep: remappable Select and Find Next/Previous (v0.3.282)

009 keep: remappable Clear all styles (shortcut_clear_all_styles, Cmd+Shift+0 default, v0.3.288)
009 keep: remappable TAB to Space / Space to TAB Leading (shortcut_tab_to_space, Cmd+Alt+V default, Shift=leading, v0.3.297)
009 keep: remappable Copy/Cut Bookmarked Lines (shortcut_copy_marked_lines, Cmd+Alt+F2 default, Shift=cut, v0.3.298)
009 keep: Find-bar Mark + Style All honors Case/Word/Re/Sel (v0.3.301)
009 keep: Find-bar Copy matching substrings (Case/Word/Re/Sel, v0.3.303)
009 keep: Find-bar Cut matching substrings (Case/Word/Re/Sel, v0.3.304)

009 keep: Find-bar Copy matching lines (Case/Word/Re/Sel, v0.3.306)

009 keep: Find-bar Shift+Purge delete matching substrings (Case/Word/Re/Sel, v0.3.308)

009 keep: Next/Prev Search Result walks Find Results hits (F4; multi-doc; wrap, v0.3.311)

009 keep: Find Results / Find in Files Enter hit jump (v0.3.315)

009 keep: remappable Find Bookmark (shortcut_find_bookmark, Alt+Shift+F4 default, v0.3.328)

009 keep: remappable Find Copy (shortcut_find_copy, Alt+Shift+F5 default; Shift=lines, v0.3.329)

009 keep: remappable Convert EOL Unix/Windows (shortcut_convert_eol, Cmd+Alt+F12 default; Shift=Windows, v0.3.340)

009 keep: remappable Sort Lines Reverse / Randomly (shortcut_sort_lines_reverse, Cmd+Alt+6 default; Shift=randomly, v0.3.341)

009 keep: remappable Sort Lines Locale Asc/Desc (shortcut_sort_lines_locale, Cmd+Alt+7 default; Shift=desc, v0.3.342)

009 keep: remappable Proper Case Force/Blend (shortcut_proper_case, Cmd+Alt+9 default; Shift=blend, v0.3.344)

009 keep: remappable Trim Leading Space (shortcut_trim_leading, Alt+Shift+T default, v0.3.347)
009 keep: remappable Convert EOL Macintosh (shortcut_convert_eol_mac, Alt+Shift+F12 default, v0.3.349)
009 keep: remappable Run (shortcut_run, Cmd+Shift+R default, v0.3.377)

009 keep: remappable Open in Default Viewer (shortcut_open_viewer, Cmd+Shift+O default, v0.3.378)

009 keep: remappable About (shortcut_about, F1 default, v0.3.379)

009 keep: Function List classic function-expression assignments (v0.3.429)

009 keep: Function List record/namespace/module/union (v0.3.430)

009 keep: Function List event/typealias/mixin/init/operator (v0.3.432)

009 keep: Function List Lua/Perl/Nim/Elixir decls (v0.3.433)

009 keep: Function List F#/OCaml let/let rec bindings (v0.3.438)

009 keep: Function List Solidity contract/library/modifier/error/constructor (v0.3.449)

009 keep: Function List MATLAB/Octave classdef (v0.3.451)

009 keep: Function List Julia baremodule/primitive type (v0.3.455)

009 keep: Function List GDScript class_name/signal (v0.3.457)

009 keep: Function List Erlang -module/-export/-record/-spec and name() -> (v0.3.459)

009 keep: Function List CSS @property/@layer/@counter-style (v0.3.461)

009 keep: Function List PowerShell filter/workflow/configuration (v0.3.462)

009 keep: Function List SystemVerilog covergroup/property/sequence/clocking (v0.3.468)

009 keep: Function List Coq Theorem/Lemma/Definition/Inductive/Fixpoint (v0.3.470)

009 keep: Function List Alloy sig/pred/fact/assert (v0.3.472)
009 keep: Function List Mercury/Prolog :- directives (v0.3.477)

009 keep: Function List Gleam opaque type (v0.3.478)

009 keep: Function List Elm port / port module / effect module (v0.3.479)

009 keep: Function List Crystal annotation / lib / alias (v0.3.480)

009 keep: Function List Apache Thrift exception/senum/typed const (v0.3.481)

009 keep: Function List WGSL var/const/override/enable/requires/diagnostic (v0.3.485)

009 keep: Function List GLSL layout/uniform/in/out/attribute/varying/shared/buffer/precision (v0.3.487)
009 keep: Function List CUDA __global__/__device__/__host__/__shared__/__constant__/__managed__ (v0.3.488)

009 keep: Function List OpenCL __kernel/kernel/__global/__local/__constant/__private (v0.3.489)

009 keep: Function List Metal vertex/fragment/kernel + device/constant/threadgroup storage (v0.3.490)

009 keep: Function List Pony be/new => / Capitalized primitive (v0.3.491)

009 keep: Function List Cython cdef/cpdef/ctypedef (v0.3.492)

009 keep: Function List Nim converter/template (v0.3.493)

009 keep: Function List Ada task/protected (v0.3.496)
009 keep: Function List Tcl oo::class create / oo::define / coroutine (v0.3.498)

009 keep: Function List CMake add_executable/add_library/add_custom_target/add_test/project (v0.3.504)

009 keep: Function List Gradle task/tasks.register/named/create/registering + Kotlin val-by (v0.3.505)

009 keep: Function List Makefile .PHONY/special targets/pattern rules/include/define (v0.3.507)
009 keep: Function List Justfile attributes/alias/set/export/import/quiet+parameterized recipes (v0.3.508)

009 keep: Function List Earthfile VERSION/FROM/SAVE/BUILD/IMPORT/GIT CLONE/LOCALLY/PROJECT/COMMAND/DO (v0.3.509)
009 keep: Function List Gentoo ebuild src_*/pkg_* phase functions (v0.3.514)
009 keep: Function List BitBake/Yocto do_* tasks (v0.3.515)
---
009 keep: Function List Android.bp/Soong modules (v0.3.516)

009 keep: Function List xmake target/option/package/rule/toolchain/task (v0.3.520)

009 keep: Function List Cabal library/executable/test-suite stanzas (v0.3.522)

009 keep: Function List pyproject.toml project/build-system/tool/dependency-groups (v0.3.524)

009 keep: Function List package.json / deno.json scripts/bin/exports/workspaces/tasks (v0.3.525)

009 keep: Function List composer.json autoload/scripts/repositories/bin/provide (v0.3.526)

009 keep: Function List go.mod directives (v0.3.527)

009 keep: Function List go.work use directives (v0.3.528)

009 keep: Function List Bundler Gemfile source/ruby/gem/group/gemspec/platform/git_source/plugin/eval_gemfile (v0.3.529)

009 keep: Function List Homebrew Brewfile tap/brew/cask/mas/whalebrew/vscode (v0.3.530)

009 keep: Function List CocoaPods Podfile directives (v0.3.531)

009 keep: Function List Pipfile (Pipenv) [[source]]/[packages]/[dev-packages]/[requires]/[scripts]/[pipenv] (v0.3.532)

009 keep: Function List systemd unit sections [Unit]/[Service]/[Install]/[Socket]/[Timer]/[Mount]/[Automount]/[Swap]/[Path]/[Slice]/[Scope] (v0.3.535)

009 keep: Function List Git config sections [core]/[user]/[remote "origin"]/[branch.main] (v0.3.537)

009 keep: Function List SSH config Host/Match/Include (v0.3.538)

009 keep: Function List WireGuard [Interface]/[Peer] (v0.3.540)

009 keep: Function List MySQL/MariaDB option-file sections (v0.3.541)

009 keep: Function List PHP.ini sections [PHP]/[CLI Server]/[Date]/[Session]/[opcache]/[MySQLi]/[mail function] (+ common extensions) (v0.3.542)

009 keep: Function List Grafana.ini sections (v0.3.549)

009 keep: Function List syslog-ng.conf source/destination/filter/parser/rewrite/template/log/options/block (v0.3.555)

009 keep: Function List dovecot.conf protocol/service/passdb/userdb/namespace/mailbox/plugin/dict/local_name/listeners/metric/local/remote (v0.3.556)

009 keep: Function List Samba smb.conf special sections (v0.3.560)

009 keep: Function List MIT Kerberos krb5.conf/kdc.conf sections (v0.3.561)

009 keep: Function List SSSD sssd.conf [sssd]/[nss]/[pam]/[sudo]/[autofs]/[ssh]/[pac]/[ifp]/[secrets]/[session_recording]/[domain/NAME] (v0.3.563)

009 keep: Function List NetworkManager connection keyfile sections (v0.3.569)

009 keep: Function List systemd-networkd .network/.netdev/.link sections (v0.3.570)

009 keep: Function List rsyslog RainerScript module/input/action/parser/timezone/template/ruleset/global/main_queue/include + $ModLoad/$IncludeConfig (v0.3.571)

009 keep: Function List Linux-PAM pam.conf/pam.d rules (v0.3.575)

009 keep: Function List sudoers rules (v0.3.576)

009 keep: Function List /etc/security/limits.conf soft/hard/- rules (v0.3.577)

009 keep: Function List /etc/security/access.conf pam_access rules (v0.3.578)

009 keep: Function List hosts.allow/hosts.deny tcp_wrappers rules (v0.3.579)

009 keep: Function List /etc/passwd and /etc/group entries (v0.3.585)

009 keep: Function List /etc/modprobe.d alias/blacklist/options/install/remove/softdep (v0.3.589)

009 keep: Function List systemd tmpfiles.d entries (v0.3.590)

009 keep: Function List /etc/services name port/proto entries (v0.3.593)


009 keep: Function List /etc/rpc name program≥100000 optional aliases (v0.3.595)

009 keep: Function List /etc/login.defs stock UPPER_SNAKE params (v0.3.598)

009 keep: Function List /etc/os-release stock KEY=VALUE fields (v0.3.599)

009 keep: Function List /etc/locale.conf LANG/LANGUAGE/LC_* (v0.3.601)

009 keep: Function List /etc/vconsole.conf KEYMAP/FONT/XKB* (v0.3.602)
