# Changelog

## [Unreleased]

## [0.3.470] — 2026-10-20

View:

- **Function List** lists Coq `Theorem` / `Lemma` / `Definition` / `Inductive` / `CoInductive` / `Fixpoint` / `CoFixpoint` / `Corollary` / `Example` / `Fact` / `Remark` / `Proposition` (`Theorem app_nil_r :`, `Definition length :=`, `Inductive nat : Type :=`, `Fixpoint map {A} (…)`; optional leading `Local` / `Global`; names may include `'`). Lowercase names need `:` / `(` / `{`. Prose and bare keywords stay out. `Record` / `Module` / `Class` stay on the generic prefix keys.

## [0.3.469] — 2026-10-20

View:

- **Function List** lists Dafny `lemma` / `predicate` / `datatype` / `codatatype` (`lemma LemmaAppend`, `predicate Sorted(s: seq<int>)`, `datatype List = Nil | Cons(...)`, `codatatype Stream = Cons(...)`; optional leading `ghost`). Lowercase names need `(` / `<` / `=` / `{`. `lemma the proof`, `predicate the property`, `datatype the list`, and bare keywords stay out. `method` / `function` / `class` / `trait` / `iterator` stay on the generic prefix keys.

## [0.3.468] — 2026-10-20

View:

- **Function List** lists SystemVerilog `covergroup` / `property` / `sequence` / `clocking` (`covergroup cg;`, `covergroup cg @(posedge clk);`, `property p_req;`, `sequence s_req;`, `clocking cb @(posedge clk);`). Builds on the existing Verilog `task` / `macromodule` / `program` / `primitive` / `checker` scan. `covergroup the bins`, `property the house`, `sequence the events`, `clocking the bus`, and bare keywords stay out.

## [0.3.467] — 2026-10-20

View:

- **Function List** lists Raku / Perl 6 `grammar` / `token` / `rule` / `regex` / `role` (`grammar JSON {`, `token number {`, `rule object {`, `regex ident {`, `role Mixable {`, `role Mixable[::T]`). Lowercase names need `{` or `[`. `grammar the json`, `token the number`, `rule the object`, `regex the ident`, `role the mixin`, and bare keywords stay out.

## [0.3.466] — 2026-10-20

View:

- **Function List** lists WASM WIT `world` / `variant` / `flags` (`world imports {`, `variant error-code {`, `flags descriptor-flags {`). Kebab-case names ok. `world the imports`, `variant the error`, `flags the bits`, and bare keywords stay out. `interface` / `record` / `func` / `package` stay on generic keys; `resource` stays on Smithy.

## [0.3.465] — 2026-10-20

View:

- **Function List** lists Smithy `operation` / `structure` / `resource` / `intEnum` (`operation GetCurrentTime {`, `structure City {`, `resource Forecast {`, `intEnum Severity {`). `operation the request`, `structure the city`, `resource the forecast`, `intEnum the severity`, and bare keywords stay out. `service` / `enum` / `union` stay on protobuf or generic keys. HCL quoted `resource "…"` stays on the HCL path.

## [0.3.464] — 2026-10-20

View:

- **Function List** lists Prisma `model` / `datasource` / `generator` / `view` (`model User {`, `datasource db {`, `generator client {`, `view UserInfo {`). `model the user`, `datasource the database`, `generator of code`, `view the mountains`, and bare keywords stay out. `enum` / `type` stay on the generic prefix keys.

## [0.3.463] — 2026-10-20

View:

- **Function List** lists FlatBuffers `table` / `rpc_service` (`table Monster {`, `rpc_service MonsterStorage {`). `table of contents`, `table the schema`, `rpc_service the storage`, and bare keywords stay out. `struct` / `enum` / `union` / `namespace` stay on the generic prefix keys.

## [0.3.462] — 2026-10-20

View:

- **Function List** lists PowerShell `filter` / `workflow` / `configuration` (`filter Get-Odd`, `filter Get-Odd($x) {`, `workflow Invoke-Job {`, `configuration WebSite {`). `filter the team`, `workflow the job`, `configuration the site`, and bare keywords stay out. `function` / `class` / `enum` stay on the generic prefix keys.

## [0.3.461] — 2026-10-20

View:

- **Function List** lists CSS `@property` / `@layer` / `@counter-style` (`@property --accent`, `@layer utilities`, `@layer theme, layout`, `@layer framework.layout`, `@counter-style thumbs`). `@property color`, `@layer {`, `@layer;`, `@counter-style the list`, and bare keywords stay out.

## [0.3.460] — 2026-10-20

View:

- **Function List** lists Sass / SCSS `@mixin` / `@function` / `@keyframes` (`@mixin flex-center`, `@mixin flex-center($dir)`, `@function rem($px)`, `@keyframes fade-in`). `@mixin the team`, `@include`, `@media`, and a bare `@mixin` / `@function` stay out.

## [0.3.459] — 2026-10-20

View:

- **Function List** lists Erlang `-module` / `-export` / `-export_type` / `-record` / `-spec` / `-callback` / `-type` / `-opaque` and `name(...) ->` clauses (`-module(foo).`, `-export([hello/0]).`, `-record(person, {name})`, `hello() -> ok.`, `inc(X) when is_integer(X) -> X.`). Bare attributes, prose, and uppercase heads stay out.

## [0.3.458] — 2026-10-20

View:

- **Function List** lists Ruby `attr_reader` / `attr_writer` / `attr_accessor` (`attr_reader :name`, `attr_accessor :x, :y`, `attr_writer(:z)`, `attr_reader :ready?`, `attr_writer :save!`). `attr_reader the name`, `attr_reader :`, and a bare `attr_reader` stay out. `def` stays on the generic prefix keys.

## [0.3.457] — 2026-10-20

View:

- **Function List** lists GDScript `class_name` / `signal` (`class_name Player`, `class_name Player extends Node2D`, `signal hit`, `signal hit(amount)`, `signal hit(amount: int)`). `class_name the player`, `signal the alarm`, and a bare `class_name` / `signal` stay out. `func` stays on the generic prefix keys.

## [0.3.456] — 2026-10-20

View:

- **Function List** lists Objective-C `@implementation` / `@protocol` (`@implementation MyClass`, `@implementation MyClass (Cat)`, `@protocol MyDelegate`, `@protocol MyDelegate <NSObject>`). `@interface` stays on the Java annotation-type path. Bare keywords and prose stay out.

## [0.3.455] — 2026-10-20

View:

- **Function List** lists Julia `baremodule` / `primitive type` (`baremodule Foo`, `primitive type Float16 16 end`, `primitive type Float16 <: AbstractFloat 16`). `baremodule the team`, `primitive type` without bits, and a bare `baremodule` stay out. `function` / `macro` / `struct` / `mutable struct` / `module` / `abstract type` stay on the generic prefix keys.

## [0.3.454] — 2026-10-20

View:

- **Function List** lists Verilog / SystemVerilog `task` / `macromodule` / `program` / `primitive` / `checker` (`task do_reset;`, `task automatic foo(`, `macromodule M;`, `program test;`, `primitive udp_and (`, `checker c1;`; optional `extern` / `export`). `task the team`, `tasks`, and a bare `task` / `program` stay out.

## [0.3.453] — 2026-10-20

View:

- **Function List** lists VHDL `entity` / `architecture` / `component` / `package` / `package body` (`entity Foo is`, `architecture RTL of Foo is`, `component Bar is`, `package Utils is`, `package body Utils is`). `entity the team`, `architecture of Foo`, `entities`, and a bare `entity` / `package` stay out.

## [0.3.452] — 2026-10-20

View:

- **Function List** lists CMake `function(name)` / `macro(name)` (`function(my_func)`, `macro(my_macro arg)`, optional space before `(`). `function()`, `functions(…)`, `endfunction()`, and a bare `function` / `macro` stay out.

## [0.3.451] — 2026-10-20

View:

- **Function List** lists MATLAB / Octave `classdef` declarations (`classdef MyClass`, `classdef MyClass < handle`, `classdef (Sealed) MyClass`). `classdef the team`, `classdefinitions`, and a bare `classdef` stay out.

## [0.3.450] — 2026-10-20

View:

- **Function List** lists Zig `const Name = struct` / `enum` / `union` / `opaque` / `error` (optional `pub`, `packed`, `extern`) and `test` blocks (`const Point = struct {`, `pub const E = enum(u8) {`, `const U = packed union {`, `test "add" {`, `test {`). `const x = 1`, `const Foo = i32`, `test the waters`, and a bare `test` stay out.

## [0.3.449] — 2026-10-20

View:

- **Function List** lists Solidity `contract`, `library`, `modifier`, `error`, and `constructor` (`contract Token is IERC20 {`, `library Math {`, `modifier onlyOwner()`, `error InsufficientBalance(uint256)`, `constructor(uint256 x)`). `contract the team`, `library of tools`, `modifier of state`, `error rate`, and a bare `constructor` stay out.

## [0.3.448] — 2026-10-20

View:

- **Function List** lists Objective-C instance and class methods (`- (void)viewDidLoad`, `+ (instancetype)shared`, `- (NSString *)title`, `- (void)initWithFrame:(NSRect)frame`). The return type is a known C or Objective-C type or an uppercase name. Markdown `- (optional) note`, `- (void) more words`, and a bare `- (void)` stay out.

## [0.3.447] — 2026-10-20

View:

- **Function List** lists Terraform / HCL blocks (`resource "aws_instance" "web" {`, `data "aws_ami" "ubuntu" {`, `variable "region" {`, `output "ip" {`, `provider "aws" {`, `locals {`, `terraform {`). `resource the team`, `variable names`, `output the log`, and a bare `locals` stay out. `data class` still lists as a class.

## [0.3.446] — 2026-10-20

View:

- **Function List** lists GraphQL `input`, `scalar`, `extend`, `fragment`, and named `query` / `mutation` / `subscription` (`input UserInput {`, `scalar DateTime`, `extend type User {`, `fragment UserFields on User {`, `query GetUser {`). `input the name`, `query the logs`, `extend the buffer`, and `fragment of text` stay out.

## [0.3.445] — 2026-10-19

View:

- **Function List** lists protobuf `message`, `service`, and `rpc` declarations (`message HelloRequest {`, `service Greeter {`, `rpc SayHello (`). `messages`, `message the team`, and `service workers` stay out.

## [0.3.444] — 2026-10-19

View:

- **Function List** lists a declaration that starts with a C++ `template<…>` header on the same line (`template<typename T> void id(`, `template<> struct Foo`, `export template<class T> class Box`, `template<typename T> concept C`). A bare `concept C =` counts. `templates`, a bare `template`, and `template<…>` with no declaration stay out.

## [0.3.443] — 2026-10-19

View:

- **Function List** lists a declaration that starts with a Java, Kotlin, or TypeScript annotation or a C# / C++ attribute on the same line (`@Override public void`, `@Composable fun`, `@Component class`, `[Fact] public void`, `[[nodiscard]] int`). Java `@interface Name` counts. A bare `@Override`, `@media`, `[section]`, and `[1, 2]` stay out.

## [0.3.442] — 2026-10-19

View:

- **Function List** lists a declaration that starts with a Rust attribute on the same line (`#[inline] pub fn`, `#[test] fn`, `#[derive(Debug)] struct`, `#[must_use] #[inline] fn`, `#[doc = "see ]"] fn`). A bare `#[derive(Debug)]`, an inner `#![no_std]`, and `#` comments stay out.

## [0.3.441] — 2026-10-19

View:

- **Function List** lists F# members (`member this.Greet(name)`, `static member Create()`, `abstract member Draw : unit -> unit`, `override this.ToString()`, `default this.Add(x)`). `member of`, `members`, and `override ToString()` stay out. Kotlin `override fun` still lists as a function.

## [0.3.440] — 2026-10-19

View:

- **Function List** lists Scala `case class` and `case object` (`case class User(name: String)`, `sealed abstract case class Expr`, `case object Empty`). Python `case Point(x):`, `case 1:`, bare `case class`, and `case classes` stay out.

## [0.3.439] — 2026-10-19

View:

- **Function List** lists parenthesized Lisp, Scheme, and Clojure definitions: `(defun …)`, `(defmacro …)`, `(defmethod …)`, `(defclass …)`, `(defn …)` / `(defn- …)`, `(define (name …)`, `(define name (lambda …)`, `(def name (fn …)`, and `(ns …)`. `(def x 1)`, `(define x 1)`, `(let …)`, `(defunct`, and `(definition` stay out.

## [0.3.438] — 2026-10-19

View:

- **Function List** lists F# / OCaml `let` / `let rec` bindings with parameters (`let greet name =`, `let private helper x =`) and `let name = fun …` / `let name = function | …`. Plain `let x = 1`, bare `let x = function`, `let!`, Rust `let f = |x| x`, and `lettuce` stay out.

## [0.3.437] — 2026-10-19

View:

- **Function List** lists R assignments `name <- function(...)`, `name <<- function(...)`, and the R 4.1 form `name <- \(...)`. Dotted names such as `read.csv` count. `x <- 1`, `x <- functional`, a bare `<- function`, and a spaced `x < - function` stay out.

## [0.3.436] — 2026-10-19

View:

- **Function List** lists SQL `CREATE TABLE` / `VIEW` / `FUNCTION` / `PROCEDURE` / `TRIGGER` / `INDEX` / `SCHEMA` / `TYPE`, including `OR REPLACE`, `TEMP` / `TEMPORARY`, `MATERIALIZED`, `UNIQUE`, and `IF NOT EXISTS`. The name may be dotted or quoted. `created`, `create a note`, a bare `CREATE TABLE`, and plural `create tables` stay out.

## [0.3.435] — 2026-10-19

View:

- **Function List** lists Elixir `defdelegate` / `defmacrop`, Clojure `defn` / `defn-` / `defonce` / `ns`, and Haskell `data` / `newtype` / `instance` when the next token is an uppercase name or `(`. Words such as `defdelegates`, `defns`, `defonces`, `nspace`, `database`, `data stays`, and `instance of` stay out. Kotlin `data class` still lists as a class.

## [0.3.434] — 2026-10-19

View:

- **Function List** lists Elixir `defprotocol` / `defimpl` / `defstruct` / `defguard` / `defguardp`, Pascal / Delphi / Ada `procedure`, and Fortran `subroutine`. Words such as `defprotocols`, `defimpls`, `defstructs`, `defguards`, `defguardps`, `procedures`, and `subroutines` stay out.

## [0.3.433] — 2026-10-19

View:

- **Function List** lists Lua `local function`, Perl `sub`, Nim `proc` / `method` / `iterator`, and Elixir `defmodule` / `defp` / `defmacro`. Words such as `substitute`, `process`, `methods`, `iterators`, `defmodules`, `defps`, and `defmacros` stay out. Bare `local x = 1` stays out.

## [0.3.432] — 2026-10-19

View:

- **Function List** lists C# `event`, Kotlin `typealias`, Dart / Kotlin `mixin`, Swift `init` / `init?` / `init!` / `deinit` / `subscript` (with `convenience` / `required` / `fileprivate`), and C++ `operator` overloads (`operator+`, `bool operator==`, `operator new` / `delete[]`, `Foo::operator()`). Words such as `events`, `typealiases`, `mixing`, `initial`, and `operators` stay out.

## [0.3.431] — 2026-10-19

View:

- **Function List** lists C# `partial` / `readonly` / `ref` / `new` qualifiers, `delegate` declarations, C `typedef`, and Go / Java `package` lines. Words such as `partially`, `delegates`, `typedefs`, and `packaged` stay out. Bare `new Foo();` stays out.

## [0.3.430] — 2026-10-19

View:

- **Function List** lists Java / C# `record` declarations, TypeScript / C# `namespace` / `module` (including `export` / `declare`), and Rust / C `union`. Words such as `recording`, `modules`, `namespaced`, and `unions` stay out.

## [0.3.429] — 2026-10-19

View:

- **Function List** lists classic JavaScript / TypeScript function-expression assignments (`const greet = function(name) {`, `let load = async function(id) {`, `var gen = function*() {`, optional expression name). `functional`, bare `function` without `(`, and `list.map(function…)` stay out.

## [0.3.428] — 2026-10-19

View:

- **Function List** lists function-like C preprocessor macros (`#define MAX(a, b) …`, `# define SQUARE(x) …`). Object-like `#define FOO 1`, `#include`, and `#ifdef` stay out.

## [0.3.427] — 2026-10-19

View:

- **Function List** lists named arrow bindings (`const greet = (name) =>`, `let load = async (id) =>`, `paint = x => x`, including a `: return` type). Bare arrows (`(x) =>`), match arms (`Ok(v) =>`), and calls stay out.

## [0.3.426] — 2026-10-19

View:

- **Function List** lists same-line method and constructor bodies (`greet() {`, `get` / `set`, `Foo::bar() {`, `~Widget() {`, `void Foo::bar() {`), `function*` generators, and `export default` / `declare` function or class lines. Calls (`foo();`), member calls, and arrows (`foo() =>`) stay out.

## [0.3.425] — 2026-10-19

View:

- **Function List** lists C, C++, Java, and C# declarations that start with a return type (`void main(`, `String name(`, `unsigned long parse(`), including `virtual` / `inline` / `constexpr`. Calls (`foo(`), assignments (`int x = f(`), and control-flow (`if`, `return`, `new`) stay out.

## [0.3.424] — 2026-10-19

View:

- **Function List** lists Swift `protocol`, `extension`, and `actor` declarations, Kotlin `suspend fun` / `data class` / `sealed` / `companion object`, and Java `final` / `abstract` class lines. A bare `object` counts only when the next token is `{`, `:`, or an uppercase name, so `object oriented` stays out.

## [0.3.423] — 2026-10-19

View:

- **Function List** lists Go and Swift `func` declarations and Kotlin `fun` declarations, including lines that start with `internal`, `override`, or `open`. `function` (JavaScript) still matches. Words such as `functionally` stay out.

## [0.3.422] — 2026-10-19

View:

- **Function List** lists Rust `macro_rules!` and `macro` items, and `fn` items behind `extern "ABI"` (including `pub unsafe extern "C" fn`). `extern crate` stays out.

## [0.3.421] — 2026-10-19

View:

- **Function List** lists Rust items that start with `pub(crate)`, `pub(super)`, `pub(in …)`, `unsafe`, or `const` (and the same stack on `struct` / `impl` / `enum` / `trait`). Comments and plain `let` lines stay out.

## [0.3.420] — 2026-10-19

Language:

- **Syntax language cycle** is remappable: Preferences → Language shortcut, or `shortcut_language` in `npp-rs/settings.json` (default `Alt+=`). Cycles the active tab through the highlight languages (plain, C, C++, HTML, Java, JavaScript, JSON, Python, Rust, SQL, XML). Shift + that chord cycles backward. Equals stays off Zoom in (`Cmd+=`) and Date Time customized (`Cmd+Alt+=`). Shortcut Mapper and About show the binding.

## [0.3.419] — 2026-10-19

Help:

- **Command Line Arguments** is remappable: Preferences → Command Line shortcut, or `shortcut_cmdline` in `npp-rs/settings.json` (default `Shift+F1`). Opens a read-only tab (? → Command Line Arguments). Shift is part of the chord so it does not steal About (`F1`), Summary (`Alt+F1`), Load Session (`Cmd+F1`), or Sort tabs by name (`Cmd+Alt+F1`). Shortcut Mapper and About show the binding.

## [0.3.418] — 2026-10-18

Help:

- **Releases** is remappable: Preferences → Releases shortcut, or `shortcut_releases` in `npp-rs/settings.json` (default `Cmd+Alt+]`). Opens GitHub Releases (? → Releases). Shift + that chord opens the changelog. The bracket stays off Indent (`Cmd+]`), Hide-equal context increase (`Alt+]`), Outdent (`Cmd+[`), and Hide-equal context decrease (`Alt+[`). Shortcut Mapper and About show the binding.

## [0.3.417] — 2026-10-18

Help:

- **Discussions** is remappable: Preferences → Discussions shortcut, or `shortcut_discussions` in `npp-rs/settings.json` (default `Cmd+Alt+[`). Opens GitHub Discussions (? → Discussions). Shift + that chord opens the Wiki. The bracket stays off Outdent (`Cmd+[`), Hide-equal context decrease (`Alt+[`), Indent (`Cmd+]`), and Hide-equal context increase (`Alt+]`). Shortcut Mapper and About show the binding.

## [0.3.416] — 2026-10-18

Help:

- **npp-rust on GitHub** is remappable: Preferences → GitHub shortcut, or `shortcut_github` in `npp-rs/settings.json` (default `Alt+;`). Opens the project repository (? → npp-rust on GitHub). Shift + that chord opens Issues. Semicolon stays off Tab colour 5 (`Cmd+;` / `Cmd+Shift+;`) and Sort tabs by modified time (`Cmd+Alt+;` / `Cmd+Alt+Shift+;`). Shortcut Mapper and About show the binding.

## [0.3.415] — 2026-10-18

Help:

- **Open npp-rust Logs** is remappable: Preferences → Open Logs shortcut, or `shortcut_open_logs` in `npp-rs/settings.json` (default `Alt+F2`). Opens `logs/*.log` from the process working directory (? → Open npp-rust Logs). Alt is part of the chord so it does not steal Next bookmark (`F2`), Toggle bookmark (`Cmd+F2`), Clear bookmarks (`Cmd+Shift+F2`), Inverse bookmarks (`Alt+Shift+F2`), or Copy bookmarked lines (`Cmd+Alt+F2`). Shortcut Mapper and About show the binding.

## [0.3.414] — 2026-10-18

Help:

- **Debug Info** is remappable: Preferences → Debug Info shortcut, or `shortcut_debug_info` in `npp-rs/settings.json` (default `Cmd+Alt+Shift+Space`). Opens a read-only tab (? → Debug Info). Shift is part of the chord so it does not steal Path autocomplete (`Cmd+Alt+Space`) or Function Call Tip (`Cmd+Shift+Space`). Shortcut Mapper and About show the binding.

## [0.3.413] — 2026-10-18

View:

- **View current file in IE** is remappable: Preferences → View in IE shortcut, or `shortcut_view_in_ie` in `npp-rs/settings.json` (default `Alt+Shift+9`). Opens the saved file in IE (View → View Current File in IE). Shift is part of the chord so it does not steal Proper Case (`Cmd+Alt+9`; Shift = blend), 1st–9th Tab (`Cmd+Shift+1`…`9`), or Fold Level (`Alt+1`…`8`). Shortcut Mapper and About show the binding.

## [0.3.412] — 2026-10-18

View:

- **View current file in Edge** is remappable: Preferences → View in Edge shortcut, or `shortcut_view_in_edge` in `npp-rs/settings.json` (default `Alt+Shift+.`). Opens the saved file in Edge (View → View Current File in Edge). Shift is part of the chord so it does not steal Sort tabs by type (`Cmd+Alt+.`; Shift = descending) or Open (`Cmd+O`). Shortcut Mapper and About show the binding.

## [0.3.411] — 2026-10-18

View:

- **View current file in Chrome** is remappable: Preferences → View in Chrome shortcut, or `shortcut_view_in_chrome` in `npp-rs/settings.json` (default `Alt+Shift+B`). Opens the saved file in Chrome (View → View Current File in Chrome). Shift is part of the chord so it does not steal Ignore Blank Lines (`Alt+B`), Begin/End Select (`Cmd+Alt+B`), or Matching brace select (`Cmd+Shift+B`). Shortcut Mapper and About show the binding.

## [0.3.410] — 2026-10-18

View:

- **View current file in Firefox** is remappable: Preferences → View in Firefox shortcut, or `shortcut_view_in_browser` in `npp-rs/settings.json` (default `Cmd+Shift+Y`). Opens the saved file in Firefox (View → View Current File in Firefox). Shift is part of the chord so it does not steal Redo (`Cmd+Y`), Copy Compare Summary (`Alt+Y`), or Save a Copy As / Rename (`Cmd+Alt+Y` / `Cmd+Alt+Shift+Y`). Shortcut Mapper and About show the binding.

## [0.3.409] — 2026-10-18

Edit:

- **Paste HTML** is remappable: Preferences → Paste HTML shortcut, or `shortcut_paste_html` in `npp-rs/settings.json` (default `Cmd+Alt+-`). Pastes clipboard text (Edit → Paste HTML Content). **Shift** flips to Paste RTF Content (`Cmd+Alt+Shift+-`). Does not steal Paste (`Cmd+V`), Paste Binary (`Cmd+Shift+V`), Zoom out (`Cmd+-`), or Date Time customized (`Cmd+Alt+=`). HTML and RTF paste stay a plain-text fallback. Shortcut Mapper and About show the binding.

## [0.3.408] — 2026-10-18

Edit:

- **Paste Binary** is remappable: Preferences → Paste Binary shortcut, or `shortcut_paste_binary` in `npp-rs/settings.json` (default `Cmd+Shift+V`). Pastes hex of UTF-8 bytes as text (Edit → Paste Binary Content). Shift is part of the chord so it does not steal Paste (`Cmd+V`), Show White Space / Show EOL (`Alt+V` / `Alt+Shift+V`), or TAB to Space / Space to TAB Leading (`Cmd+Alt+V` / `Cmd+Alt+Shift+V`). Shortcut Mapper and About show the binding.

## [0.3.407] — 2026-10-18

Plugins / Ollama (issue #21):

- **Ollama Chat** panel: shows the exact prompt sent, a **Thinking…** indicator with elapsed seconds while the worker runs, and the reply in a chat transcript. Follow-up messages via Send; optional attach of selection/buffer. **Open reply in tab** is optional.
- **Ask Ollama** (Plugins and editor right-click) opens that panel and uses Ollama `/api/chat` so multi-turn context stays in the panel. Status still lists models in a tab.

## [0.3.406] — 2026-10-18

Edit:

- **Cut Binary** is remappable: Preferences → Cut Binary shortcut, or `shortcut_cut_binary` in `npp-rs/settings.json` (default `Cmd+Alt+Shift+X`). Cuts the selection as hex of its UTF-8 bytes (Edit → Cut Binary Content). Does not steal Cut (`Cmd+X`), Copy Binary (`Cmd+Shift+X`), Space to TAB (All) (`Cmd+Alt+X`), or Collapse unchanged at caret (`Alt+Shift+X`). Shortcut Mapper and About show the binding.

## [0.3.405] — 2026-10-18

Edit:

- **Copy Binary** is remappable: Preferences → Copy Binary shortcut, or `shortcut_copy_binary` in `npp-rs/settings.json` (default `Cmd+Shift+X`). Copies the selection as hex of its UTF-8 bytes (Edit → Copy Binary Content). Shift is part of the chord so it does not steal Cut (`Cmd+X`), Space to TAB (All) (`Cmd+Alt+X`), or Collapse unchanged at caret (`Alt+Shift+X`). Shortcut Mapper and About show the binding.

## [0.3.404] — 2026-10-18

Edit:

- **Copy / Cut / Paste Binary** uses hex of the selection’s UTF-8 bytes (`48 69` for `Hi`). Paste binary decodes that hex back into text. HTML and RTF paste stay a plain-text fallback. A non-hex clipboard is left unchanged.

## [0.3.403] — 2026-10-18

Edit:

- **System read-only attribute** is remappable: Preferences → System read-only attribute shortcut, or `shortcut_system_readonly` in `npp-rs/settings.json` (default `Cmd+Alt+Shift+F`). Toggles the OS read-only bit on the saved file and the app read-only flag (Edit → Read-Only Attribute in Windows). Shift is part of the chord so it does not steal Find (`Cmd+F`), Replace alternate (`Cmd+Shift+F`), Find in Files (`Cmd+Alt+F`), or Fold current (`Alt+F` / `Alt+Shift+F`). Shortcut Mapper and About show the binding.

## [0.3.402] — 2026-10-18

Tools:

- **SHA-512 generate from file** is remappable: Preferences → SHA-512 from file shortcut, or `shortcut_sha512_file` in `npp-rs/settings.json` (default `Cmd+Shift+F11`). Hashes the saved file and copies the digest (Tools → SHA-512 → Generate from files). Shift is part of the chord so it does not steal SHA-512 generate (`Cmd+F6` / `Cmd+Shift+F6`) or ANSI (`Cmd+F11`). Shortcut Mapper and About show the binding.

## [0.3.401] — 2026-10-18

Tools:

- **SHA-1 generate from file** is remappable: Preferences → SHA-1 from file shortcut, or `shortcut_sha1_file` in `npp-rs/settings.json` (default `Alt+F12`). Hashes the saved file and copies the digest (Tools → SHA-1 → Generate from files). Does not steal Post-It (`F12`), SHA-1 generate (`Cmd+F12` / `Cmd+Shift+F12`), Convert EOL Macintosh (`Alt+Shift+F12`), or Convert EOL Unix/Windows (`Cmd+Alt+F12`). Shortcut Mapper and About show the binding.

## [0.3.400] — 2026-10-18

Tools:

- **SHA-256 generate from file** is remappable: Preferences → SHA-256 from file shortcut, or `shortcut_sha256_file` in `npp-rs/settings.json` (default `Alt+Shift+H`). Hashes the saved file and copies the digest (Tools → SHA-256 → Generate from files). Shift is part of the chord so it does not steal Hide unchanged lines (`Alt+H`) or SHA-256 generate (`Alt+F11` / `Alt+Shift+F11`). Shortcut Mapper and About show the binding.

## [0.3.399] — 2026-10-18

Tools:

- **MD5 generate from file** is remappable: Preferences → MD5 from file shortcut, or `shortcut_hash_file` in `npp-rs/settings.json` (default `Cmd+Shift+H`). Hashes the saved file and copies the digest (Tools → MD5 → Generate from files). Shift is part of the chord so it does not steal Replace (`Cmd+H`) or Hide unchanged lines (`Alt+H`). Shortcut Mapper and About show the binding.

## [0.3.398] — 2026-10-18

Edit:

- **Clipboard History** keeps the last 10 copies from this session. Edit → Clipboard History (default `Cmd+Shift+C`, Preferences `shortcut_clipboard_history`) cycles them into the paste clipboard. The status line shows the slot (`2/10`) and a short preview. There is still no history panel.

## [0.3.397] — 2026-10-18

Search:

- **Find Mark Style** next, previous, and copy use style 1 (the slot Find-bar Mark writes). They no longer walk or copy bookmarks. Menu rows show the style 1 chords from Next Style and Copy Styled Text.

## [0.3.396] — 2026-10-18

Edit:

- **Multi-select All (Match Case + Whole Word)** is remappable: Preferences → Multi-select all match case whole word shortcut, or `shortcut_multi_select_all_case_word` in `npp-rs/settings.json` (default `Alt+Shift+A`). Selects every case-sensitive whole-word match of the current word or selection as multi-carets (Edit → Multi-select All → Match Case Whole Word). Shift is part of the chord so it does not steal Show All Characters (`Alt+A`), Select All (`Cmd+A`), Multi-select all match case (`Cmd+Shift+A`), or Multi-select all / whole word (`Cmd+Alt+A` / `Cmd+Alt+Shift+A`). Shortcut Mapper and About show the binding.

## [0.3.395] — 2026-10-18

Edit:

- **Multi-select All (Match Case)** is remappable: Preferences → Multi-select all match case shortcut, or `shortcut_multi_select_all_case` in `npp-rs/settings.json` (default `Cmd+Shift+A`). Selects every case-sensitive match of the current word or selection as multi-carets (Edit → Multi-select All → Match Case Only). Shift is part of the chord so it does not steal Select All (`Cmd+A`), Show All Characters (`Alt+A`), or Multi-select all / whole word (`Cmd+Alt+A` / `Cmd+Alt+Shift+A`). Shortcut Mapper and About show the binding.

## [0.3.394] — 2026-10-18

Edit:

- **Multi-select Next (Match Case + Whole Word)** is remappable: Preferences → Multi-select next match case whole word shortcut, or `shortcut_multi_select_next_case_word` in `npp-rs/settings.json` (default `Cmd+Shift+N`). Adds the next case-sensitive whole-word match as a caret (Edit → Multi-select Next → Match Case Whole Word). Shift is part of the chord so it does not steal New (`Cmd+N`), Show NPC (`Alt+N`), Multi-select next / skip (`Cmd+Alt+N` / `Cmd+Alt+Shift+N`), or Match Case Only (`Alt+Shift+N`). Shortcut Mapper and About show the binding.

## [0.3.393] — 2026-10-18

Edit:

- **Multi-select Next (Whole Word)** is remappable: Preferences → Multi-select next whole word shortcut, or `shortcut_multi_select_next_word` in `npp-rs/settings.json` (default `Cmd+Shift+W`). Adds the next whole-word match as a caret (Edit → Multi-select Next → Whole Word). Shift is part of the chord so it does not steal Close tab (`Cmd+W`), Close All (`Cmd+Alt+W`), or Ignore whitespace (`Alt+W`). Shortcut Mapper and About show the binding.

## [0.3.392] — 2026-10-18

Edit:

- **Multi-select Next (Match Case)** is remappable: Preferences → Multi-select next match case shortcut, or `shortcut_multi_select_next_case` in `npp-rs/settings.json` (default `Alt+Shift+N`). Adds the next case-sensitive match as a caret (Edit → Multi-select Next → Match Case Only). Shift is part of the chord so it does not steal Show NPC (`Alt+N`) or Multi-select next / skip (`Cmd+Alt+N` / `Cmd+Alt+Shift+N`). Shortcut Mapper and About show the binding.

## [0.3.391] — 2026-10-18

Edit:

- **Block Comment / Block Uncomment** is remappable: Preferences → Block comment shortcut, or `shortcut_stream_comment` in `npp-rs/settings.json` (default `Alt+,`). Wraps the selection in a block comment (Edit → Comment/Uncomment → Block Comment). Shift flips to Block Uncomment. Does not steal Toggle comment (`Cmd+/` / `Cmd+Shift+/`), Single Line Comment (`Alt+/`), Preferences (`Cmd+,`), or Sort tabs by path (`Cmd+Alt+,`). Shortcut Mapper and About show the binding.

## [0.3.390] — 2026-10-17

Edit:

- **Single Line Comment / Uncomment** is remappable: Preferences → Single line comment shortcut, or `shortcut_line_comment` in `npp-rs/settings.json` (default `Alt+/`). Forces a line comment (Edit → Comment/Uncomment → Single Line Comment). Shift flips to Single Line Uncomment. Does not steal Toggle comment (`Cmd+/`), Block Comment (`Cmd+Shift+/`), or Sort tabs by size (`Cmd+Alt+/`). Shortcut Mapper and About show the binding.

## [0.3.389] — 2026-10-17

Macro:

- **Run a Macro Multiple Times** is remappable: Preferences → Run macro multiple times shortcut, or `shortcut_macro_repeat` in `npp-rs/settings.json` (default `Cmd+Alt+Shift+K`). Replays the recorded steps three times (Macro → Run a Macro Multiple Times). Shift is part of the chord so it does not steal Copy Current Directory (`Cmd+Alt+K`), Copy Compare Hunk (`Alt+K`), Shortcut Mapper (`Alt+Shift+K`), or Tab colour 3 (`Cmd+K`). Shortcut Mapper and About show the binding.

## [0.3.388] — 2026-10-17

Macro:

- **Save Current Recorded Macro** is remappable: Preferences → Save recorded macro shortcut, or `shortcut_macro_save` in `npp-rs/settings.json` (default `Cmd+Alt+Shift+S`). Writes the recorded steps (Macro → Save Current Recorded Macro). Shift is part of the chord so it does not steal Save (`Cmd+S`), Save As (`Cmd+Shift+S`), Save All (`Cmd+Alt+S`), Swap Compare (`Alt+S`), or Compare to Saved (`Alt+Shift+S`). Shortcut Mapper and About show the binding.

## [0.3.387] — 2026-10-17

Macro:

- **Start/Stop Recording** is remappable: Preferences → Start/Stop Recording shortcut, or `shortcut_macro_record` in `npp-rs/settings.json` (default `Cmd+Alt+Shift+R`). The same chord starts recording when idle and stops while recording (Macro → Start Recording / Stop Recording). Shift is part of the chord so it does not steal Reload (`Cmd+R`), Run (`Cmd+Shift+R`), Toggle Read-Only (`Cmd+Alt+R`), or RTL/LTR (`Alt+R` / `Alt+Shift+R`). Shortcut Mapper and About show the binding.

## [0.3.386] — 2026-10-17

Macro:

- **Playback** is remappable: Preferences → Macro Playback shortcut, or `shortcut_macro_playback` in `npp-rs/settings.json` (default `Cmd+Shift+P`). Plays the recorded macro (Macro → Playback). Shift is part of the chord so it does not steal Print (`Cmd+P`). Shortcut Mapper and About show the binding.

## [0.3.385] — 2026-10-17

Edit:

- **Date Time (customized)** is remappable: Preferences → Date Time customized shortcut, or `shortcut_insert_datetime_custom` in `npp-rs/settings.json` (default `Cmd+Alt+=`). Inserts the ISO-style local timestamp (Edit → Insert). Does not collide with Date Time short/long (`F5` / `Shift+F5`) or Zoom in (`Cmd+=`). Shortcut Mapper and About show the binding.

## [0.3.384] — 2026-10-17

View:

- **1st–9th Tab** is remappable: Preferences → 1st–9th tab shortcut, or `shortcut_goto_tab` in `npp-rs/settings.json` (default `Cmd+Shift+1`). Digit keys 1 through 9 with those modifiers jump to View → Tab → 1st…9th Tab. A letter remap jumps to the 1st tab only. Does not collide with Mark Style (`Cmd+1`…`5`), Fold Level (`Alt+1`…`8`), or Clear all styles (`Cmd+Shift+0`). Shortcut Mapper and About show the binding.

## [0.3.383] — 2026-10-17

View:

- **Move to New Instance / Open in New Instance** is remappable: Preferences → New instance shortcut, or `shortcut_new_instance` in `npp-rs/settings.json` (default `F9`). Shift opens another window and keeps this tab. Does not collide with Find characters in range (`Alt+F9`), UTF-8 (`Cmd+F9`), read-only for all (`F10`), fullscreen (`F11`), or Post-It (`F12`). Shortcut Mapper and About show both bindings.

## [0.3.382] — 2026-10-17

Edit:

- **Undo the latest added multi-select** is remappable: Preferences → Multi-select undo shortcut, or `shortcut_multi_select_undo` in `npp-rs/settings.json` (default `Cmd+Alt+Backspace`). Drops the newest extra caret. Does not also delete a character, and does not collide with Undo (`Cmd+Z`) or bare Backspace. Shortcut Mapper and About show the binding.

## [0.3.381] — 2026-10-17

Edit:

- **Function Parameters Next / Previous** is remappable: Preferences → Function parameters shortcut, or `shortcut_calltip_nav` in `npp-rs/settings.json` (default `Alt+Down`). Steps the in-file function call tip. Opposite Up is the previous hint. Letter remaps use Shift for previous. Does not collide with Move Line (`Cmd+Shift+Up`/`Down`), plain arrow motion, or rect select (`Alt+Shift+arrows`). Shortcut Mapper and About show both bindings.

## [0.3.380] — 2026-10-17

File:

- **Move to Recycle Bin** is remappable: Preferences → Move to Recycle Bin shortcut, or `shortcut_delete_file` in `npp-rs/settings.json` (default `Cmd+Shift+Delete`). Moves the saved active file to the user Trash and closes the tab. Does not also delete a character, and does not collide with forward Delete. Shortcut Mapper and About show the binding.

## [0.3.379] — 2026-10-17

Help:

- **About** is remappable: Preferences → About shortcut, or `shortcut_about` in `npp-rs/settings.json` (default `F1`). Opens ? → About npp-rs. Does not collide with Summary (`Alt+F1`), Load Session (`Cmd+F1`), or Sort tabs by name (`Cmd+Alt+F1`). Shortcut Mapper and About show the binding.

## [0.3.378] — 2026-10-17

File:

- **Open in Default Viewer** is remappable: Preferences → Open in Default Viewer shortcut, or `shortcut_open_viewer` in `npp-rs/settings.json` (default `Cmd+Shift+O`). Opens the active file in the OS default viewer. Does not collide with Open (`Cmd+O`), Move/Clone Other View (`Alt+O`), or Sort Lines (`Cmd+Alt+O`). Shortcut Mapper and About show the binding.

## [0.3.377] — 2026-10-17

Run:

- **Run…** is remappable: Preferences → Run shortcut, or `shortcut_run` in `npp-rs/settings.json` (default `Cmd+Shift+R`). Runs Run → Run… (pick a program to spawn; cancel opens a shell in the active file’s folder). Does not collide with Reload (`Cmd+R`), Toggle Read-Only (`Cmd+Alt+R`), or RTL (`Alt+R`). Shortcut Mapper and About show the binding.

## [0.3.376] — 2026-10-17

Edit:

- **Clipboard History** is remappable: Preferences → Clipboard History shortcut, or `shortcut_clipboard_history` in `npp-rs/settings.json` (default `Cmd+Shift+C`). Runs Edit → Clipboard History (shows the last copied entry in the status line; 1-entry MVP). Does not collide with Copy (`Cmd+C`), Column Editor (`Cmd+Alt+C`), or Copy Compare Diff (`Alt+C`). Shortcut Mapper and About show the binding.

## [0.3.375] — 2026-10-17

View:

- **Tab colour 5 / clear** is remappable: Preferences → Tab colour 5 shortcut, or `shortcut_tab_colour_5` in `npp-rs/settings.json` (default `Cmd+;`). Sets View → Apply Color 5 on the active tab. Shift + that chord removes the tab colour. Does not collide with Sort tabs by modified (`Cmd+Alt+;`), Preferences (`Cmd+,`), or Tab colour 4 (`Cmd+M`). Shortcut Mapper and About show both bindings.

## [0.3.374] — 2026-10-17

View:

- **Tab colour 4 / clear** is remappable: Preferences → Tab colour 4 shortcut, or `shortcut_tab_colour_4` in `npp-rs/settings.json` (default `Cmd+M`). Sets View → Apply Color 4 on the active tab. Shift + that chord removes the tab colour. Does not collide with Bookmark Compare Diffs (`Alt+M`), Copy All Paths (`Cmd+Alt+M`), or Tab colour 3 (`Cmd+K`). Shortcut Mapper and About show both bindings.

## [0.3.373] — 2026-10-17

View:

- **Tab colour 3 / clear** is remappable: Preferences → Tab colour 3 shortcut, or `shortcut_tab_colour_3` in `npp-rs/settings.json` (default `Cmd+K`). Sets View → Apply Color 3 on the active tab. Shift + that chord removes the tab colour. Does not collide with Copy Compare Hunk (`Alt+K`), Shortcut Mapper (`Alt+Shift+K`), or Copy Current Directory (`Cmd+Alt+K`). Shortcut Mapper and About show both bindings.

## [0.3.372] — 2026-10-17

View:

- **Tab colour 2 / clear** is remappable: Preferences → Tab colour 2 shortcut, or `shortcut_tab_colour_2` in `npp-rs/settings.json` (default `Cmd+F4`). Sets View → Apply Color 2 on the active tab. Shift + that chord removes the tab colour. Does not collide with Next/Prev Search Result (`F4`), Find All (`Cmd+Alt+F4`), or Find Bookmark (`Alt+Shift+F4`). Shortcut Mapper and About show both bindings.

## [0.3.371] — 2026-10-17

View:

- **Tab colour 1 / clear** is remappable: Preferences → Tab colour shortcut, or `shortcut_tab_colour` in `npp-rs/settings.json` (default `Cmd+F5`). Sets View → Apply Color 1 on the active tab. Shift + that chord removes the tab colour. Does not collide with Date Time (`F5`), Sort Lines Integer (`Cmd+Alt+F5`), or Find Copy (`Alt+Shift+F5`). Shortcut Mapper and About show both bindings.

## [0.3.370] — 2026-10-17

Settings:

- **Shortcut Mapper** is remappable: Preferences → Shortcut Mapper shortcut, or `shortcut_mapper` in `npp-rs/settings.json` (default `Alt+Shift+K`). Opens Settings → Shortcut Mapper (static dump of hard-wired and remappable chords). Does not collide with Copy Compare Hunk (`Alt+K`), Copy Current Directory Path (`Cmd+Alt+K`), or Preferences (`Cmd+,`). Shortcut Mapper and About show the binding.

## [0.3.369] — 2026-10-17

Tools:

- **SHA-512 generate / clipboard** is remappable: Preferences → SHA-512 shortcut, or `shortcut_sha512` in `npp-rs/settings.json` (default `Cmd+F6`). Hashes the selection, or the whole document, and inserts the digest (Tools → SHA-512 → Generate). Shift + that chord copies the digest instead. Does not collide with Focus on Another View (`F6`), Find Cut (`Alt+Shift+F6`), or Sort Lines Lex Ignoring Case (`Cmd+Alt+F6`). Shortcut Mapper and About show both bindings.

## [0.3.368] — 2026-10-17

Tools:

- **SHA-1 generate / clipboard** is remappable: Preferences → SHA-1 shortcut, or `shortcut_sha1` in `npp-rs/settings.json` (default `Cmd+F12`). Hashes the selection, or the whole document, and inserts the digest (Tools → SHA-1 → Generate). Shift + that chord copies the digest instead. Does not collide with Post-It (`F12`), Convert EOL Macintosh (`Alt+Shift+F12`), or Convert EOL Unix/Windows (`Cmd+Alt+F12`). Shortcut Mapper and About show both bindings.

## [0.3.367] — 2026-10-17

Tools:

- **SHA-256 generate / clipboard** is remappable: Preferences → SHA-256 shortcut, or `shortcut_sha256` in `npp-rs/settings.json` (default `Alt+F11`). Hashes the selection, or the whole document, and inserts the digest (Tools → SHA-256 → Generate). Shift + that chord copies the digest instead. Does not collide with Fullscreen (`F11`), ANSI (`Cmd+F11`), or EOL to Space (`Cmd+Alt+F11`). Shortcut Mapper and About show both bindings.

## [0.3.366] — 2026-10-17

Tools:

- **MD5 generate / clipboard** is remappable: Preferences → MD5 shortcut, or `shortcut_md5` in `npp-rs/settings.json` (default `Alt+F10`). Hashes the selection, or the whole document, and inserts the digest (Tools → MD5 → Generate). Shift + that chord copies the digest instead. Does not collide with Read-only for all documents (`F10` / Shift+F10), UTF-16 (`Cmd+F10`), or Sort Lines by Length (`Cmd+Alt+F10`). Shortcut Mapper and About show both bindings.

## [0.3.365] — 2026-10-17

Encoding:

- **ANSI** is remappable: Preferences → ANSI shortcut, or `shortcut_encoding_ansi` in `npp-rs/settings.json` (default `Cmd+F11`). Sets the active tab to ANSI (Encoding → ANSI; save writes Windows-1252). Does not collide with Fullscreen (`F11`) or EOL to Space (`Cmd+Alt+F11`). Shortcut Mapper and About show the binding.

## [0.3.364] — 2026-10-17

Encoding:

- **UTF-16 LE / UTF-16 BE** is remappable: Preferences → UTF-16 shortcut, or `shortcut_encoding_utf16` in `npp-rs/settings.json` (default `Cmd+F10`). Sets the active tab to UTF-16 LE BOM (Encoding → UTF-16 LE BOM). Shift + that chord sets UTF-16 BE BOM. Does not collide with Read-only for all documents (`F10` / Shift+F10) or Sort Lines by Length (`Cmd+Alt+F10`). Shortcut Mapper and About show both bindings.

## [0.3.363] — 2026-10-16

Encoding:

- **UTF-8 / UTF-8-BOM** is remappable: Preferences → UTF-8 shortcut, or `shortcut_encoding_utf8` in `npp-rs/settings.json` (default `Cmd+F9`). Sets the active tab to UTF-8 without BOM (Encoding → UTF-8). Shift + that chord sets UTF-8-BOM. Does not collide with Find characters in range (`Alt+F9`) or Sort Lines Decimal Comma (`Cmd+Alt+F9`). Shortcut Mapper and About show both bindings.

## [0.3.362] — 2026-10-16

Window:

- **Sort tabs by modified time** is remappable: Preferences → Sort tabs by modified time shortcut, or `shortcut_sort_tabs_modified` in `npp-rs/settings.json` (default `Cmd+Alt+;`). Sorts open tabs by file modified time, oldest first (Window, Modified Time Ascending). Shift + that chord sorts newest first. Does not collide with Sort tabs by size (`Cmd+Alt+/`) or Toggle comment (`Cmd+/`). Shortcut Mapper and About show both bindings.

## [0.3.361] — 2026-10-16

Window:

- **Sort tabs by size** is remappable: Preferences → Sort tabs by size shortcut, or `shortcut_sort_tabs_size` in `npp-rs/settings.json` (default `Cmd+Alt+/`). Sorts open tabs by buffer size, smallest first (Window, Size Ascending). Shift + that chord sorts largest first. Does not collide with Sort tabs by type (`Cmd+Alt+.`) or Toggle comment (`Cmd+/`). Shortcut Mapper and About show both bindings.

## [0.3.360] — 2026-10-16

Window:

- **Sort tabs by type** is remappable: Preferences → Sort tabs by type shortcut, or `shortcut_sort_tabs_type` in `npp-rs/settings.json` (default `Cmd+Alt+.`). Sorts open tabs by type A to Z (Window, Type A to Z). Shift + that chord sorts Z to A. Does not collide with Sort tabs by path (`Cmd+Alt+,`) or Preferences (`Cmd+,`). Shortcut Mapper and About show both bindings.

## [0.3.359] — 2026-10-16

Window:

- **Sort tabs by path** is remappable: Preferences → Sort tabs by path shortcut, or `shortcut_sort_tabs_path` in `npp-rs/settings.json` (default `Cmd+Alt+,`). Sorts open tabs by path A to Z (Window, Path A to Z). Shift + that chord sorts Z to A. Does not collide with Sort tabs by name (`Cmd+Alt+F1`) or Preferences (`Cmd+,`). Shortcut Mapper and About show both bindings.

## [0.3.358] — 2026-10-16

Window:

- **Sort tabs by name** is remappable: Preferences → Sort tabs by name shortcut, or `shortcut_sort_tabs` in `npp-rs/settings.json` (default `Cmd+Alt+F1`). Sorts open tabs A to Z (Window, Name A to Z). Shift + that chord sorts Z to A. Does not collide with Load Session (`Cmd+F1`) or Summary (`Alt+F1`). Shortcut Mapper and About show both bindings.

## [0.3.357] — 2026-10-16

File:

- **Load Session / Save Session** is remappable: Preferences → Load Session shortcut, or `shortcut_load_session` in `npp-rs/settings.json` (default `Cmd+F1`). Loads the saved session (File → Load Session). Shift + that chord saves the session. Does not collide with Summary (`Alt+F1`) or Find (`Cmd+F`). Shortcut Mapper and About show both bindings.

## [0.3.356] — 2026-10-16

File:

- **Exit** is remappable: Preferences → Exit shortcut, or `shortcut_exit` in `npp-rs/settings.json` (default `Cmd+Q`). Quits (File → Exit) and still prompts when a document is dirty. Does not collide with Paste Bookmarked Lines (`Cmd+Alt+Q`) or Sync Vertical Scroll (`Alt+Q`). Toggle Comment stays `Cmd+/`. Shortcut Mapper and About show the binding.

## [0.3.355] — 2026-10-16

Settings:

- **Preferences / Style Configurator** is remappable: Preferences → Preferences shortcut, or `shortcut_preferences` in `npp-rs/settings.json` (default `Cmd+,`). Opens Settings → Preferences. Shift + that chord opens Style Configurator. Does not collide with Summary (`Alt+F1`), Print (`Cmd+P`), or Project Panel (`Alt+P`). Shortcut Mapper and About show both bindings.

## [0.3.354] — 2026-10-16

View:

- **Summary / Document List** is remappable: Preferences → Summary shortcut, or `shortcut_summary` in `npp-rs/settings.json` (default `Alt+F1`). Opens View → Summary. Shift + that chord opens View → Document List. Does not collide with Find characters in range (`Alt+F9`) or Read-only for all documents (`F10`). Shortcut Mapper and About show both bindings.

## [0.3.353] — 2026-10-16

Edit:

- **Read-only for all documents** is remappable: Preferences → Read-only for all documents shortcut, or `shortcut_readonly_all` in `npp-rs/settings.json` (default `F10`). Sets read-only on every open document. Shift + that chord clears read-only on every open document. Does not collide with Toggle Read-Only (`Cmd+Alt+R`), RTL (`Alt+R`), Reload (`Cmd+R`), or Sort Lines by Length (`Cmd+Alt+F10`). Shortcut Mapper and About show the effective binding.

## [0.3.352] — 2026-10-16

View:

- **Show Wrap Symbol** is remappable: Preferences → Show wrap symbol shortcut, or `shortcut_show_wrap_symbol` in `npp-rs/settings.json` (default `Alt+Shift+G`). Runs View → Show Wrap Symbol. Does not collide with Show Fold Margin (`Alt+G`), Open Folder as Workspace (`Cmd+Alt+G`), word wrap (`Alt+Z`), or Zoom Sync (`Alt+Shift+Z`). Shortcut Mapper and About show the effective binding.

## [0.3.351] — 2026-10-16

View:

- **Show Wrap Symbol** is a real toggle: View → Show Wrap Symbol, or Preferences → Show wrap symbol, or `show_wrap_symbol` in `npp-rs/settings.json`. When on (the default), a wrapped line draws a wrap mark. Turning it off keeps word wrap and hides the mark. The menu shows a check when the mark is on.

## [0.3.350] — 2026-10-16

View:

- **Hide Lines** is remappable: Preferences → Hide lines shortcut, or `shortcut_hide_lines` in `npp-rs/settings.json` (default `Alt+Shift+L`). Runs View → Hide Lines (current line, or the selected line range). Does not collide with Show Line Numbers (`Alt+L`), Go to line (`Cmd+L`), Delete line (`Cmd+Shift+L`), or Open Containing Folder (`Cmd+Alt+L`). Shortcut Mapper and About show the effective binding.

## [0.3.349] — 2026-10-16

Format:

- **Convert EOL Macintosh (CR)** is remappable: Preferences → Convert EOL Macintosh shortcut, or `shortcut_convert_eol_mac` in `npp-rs/settings.json` (default `Alt+Shift+F12`). Runs Format → Convert to Macintosh. Does not collide with Post-It (`F12`), Convert EOL Unix/Windows (`Cmd+Alt+F12` / `Cmd+Alt+Shift+F12`), or Find Purge (`Alt+Shift+F8`). Shortcut Mapper and About show the effective binding.

## [0.3.348] — 2026-10-16

Edit:

- **Search on Internet / Change Search Engine** is remappable: Preferences → Search on Internet shortcut, or `shortcut_search_internet` in `npp-rs/settings.json` (default `Alt+U`). Runs Edit → Search on Internet (selection required). **Shift** flips to Change Search Engine. Does not collide with Invert Case (`Cmd+Alt+U`), lowercase (`Cmd+U`), or Redact Selection (`Alt+J`). Shortcut Mapper and About show both bindings.

## [0.3.347] — 2026-10-16

Edit:

- **Trim Leading Space** is remappable: Preferences → Trim Leading shortcut, or `shortcut_trim_leading` in `npp-rs/settings.json` (default `Alt+Shift+T`). Runs Edit → Blank Operations → Trim Leading Space. Does not collide with Trim Trailing (`Cmd+Alt+T`), Always on Top (`Alt+T`), or Toggle log tail (`Cmd+Shift+T`). Shortcut Mapper and About show the effective binding.

## [0.3.346] — 2026-10-16

Edit:

- **Redact Selection** is remappable: Preferences → Redact Selection shortcut, or `shortcut_redact` in `npp-rs/settings.json` (default `Alt+J`). Runs Edit → On Selection → Redact Selection █. **Shift** flips to ● fill (matches the menu label). Does not collide with Join Lines (`Cmd+J`), RTL (`Alt+R`), Toggle Read-Only (`Cmd+Alt+R`), or Invert Case (`Cmd+Alt+U`). Shortcut Mapper and About show both bindings.

## [0.3.345] — 2026-10-16

Edit:

- **Random Case** is remappable: Preferences → Random Case shortcut, or `shortcut_random_case` in `npp-rs/settings.json` (default `Cmd+Alt+0`). Runs Edit → Convert Case to → Random Case. Does not collide with Fold All (`Alt+0`), Copy Styled Text all (`Cmd+Alt+Shift+0`), Proper Case (`Cmd+Alt+9`), Next Style (`Cmd+Alt+1`…`5`), or Fold Level (`Alt+1`…`8`). Shortcut Mapper and About show the effective binding.

## [0.3.344] — 2026-10-16

Edit:

- **Proper Case Force / Blend** is remappable: Preferences → Proper Case shortcut, or `shortcut_proper_case` in `npp-rs/settings.json` (default `Cmd+Alt+9`). Runs Edit → Convert Case to → Proper Case. **Shift** flips to Proper Case blend. Invert Case + Shift still aliases Proper Case force. Does not collide with Sentence Case (`Cmd+Alt+8`), Next Style (`Cmd+Alt+1`…`5`), or Fold Level (`Alt+1`…`8`). Shortcut Mapper and About show the effective bindings.

## [0.3.343] — 2026-10-16

Edit:

- **Sentence Case Force / Blend** is remappable: Preferences → Sentence Case shortcut, or `shortcut_sentence_case` in `npp-rs/settings.json` (default `Cmd+Alt+8`). Runs Edit → Convert Case to → Sentence case. **Shift** flips to Sentence case blend. Does not collide with Sort Lines Locale (`Cmd+Alt+7`), Invert Case (`Cmd+Alt+U`), Next Style (`Cmd+Alt+1`…`5`), or Fold Level (`Alt+1`…`8`). Shortcut Mapper and About show the effective bindings.

## [0.3.342] — 2026-10-16

Edit:

- **Sort Lines Locale Asc / Desc** is remappable: Preferences → Sort Lines Locale shortcut, or `shortcut_sort_lines_locale` in `npp-rs/settings.json` (default `Cmd+Alt+7`). Runs Edit → Line Operations → Sort Lines Locale Ascending. **Shift** flips to descending. Does not collide with Sort Lines Reverse (`Cmd+Alt+6`), Next Style (`Cmd+Alt+1`…`5`), or Fold Level (`Alt+1`…`8`). Shortcut Mapper and About show the effective bindings.

## [0.3.341] — 2026-10-16

Edit:

- **Sort Lines Reverse Order / Randomly** is remappable: Preferences → Sort Lines Reverse shortcut, or `shortcut_sort_lines_reverse` in `npp-rs/settings.json` (default `Cmd+Alt+6`). Runs Edit → Line Operations → Sort Lines in Reverse Order. **Shift** flips to Sort Lines Randomly. Does not collide with Next Style (`Cmd+Alt+1`…`5`) or Fold Level (`Alt+1`…`8`). Shortcut Mapper and About show the effective bindings.

## [0.3.340] — 2026-10-16

Format:

- **Convert EOL Unix (LF) / Windows (CRLF)** is remappable: Preferences → Convert EOL shortcut, or `shortcut_convert_eol` in `npp-rs/settings.json` (default `Cmd+Alt+F12`). Runs Format → Convert to Unix. **Shift** flips to Convert to Windows. Macintosh (CR) stays menu-only. Does not collide with Post-It (`F12`) or EOL to Space (`Cmd+Alt+F11`). Shortcut Mapper and About show the effective bindings.

## [0.3.339] — 2026-10-16

Edit:

- **EOL to Space / Trim Both and EOL to Space** is remappable: Preferences → EOL to Space shortcut, or `shortcut_eol_to_space` in `npp-rs/settings.json` (default `Cmd+Alt+F11`). Runs Edit → Blank Operations → EOL to Space. **Shift** flips to Trim Both and EOL to Space. Does not collide with Fullscreen (`F11`), Sort Lines by Length (`Cmd+Alt+F10`), or Trim Trailing (`Cmd+Alt+T`). Shortcut Mapper and About show the effective bindings.

## [0.3.338] — 2026-10-16

Edit:

- **Sort Lines by Length Asc / Desc** is remappable: Preferences → Sort Lines by Length shortcut, or `shortcut_sort_lines_length` in `npp-rs/settings.json` (default `Cmd+Alt+F10`). Runs Edit → Line Operations → Sort Lines by Length Ascending. **Shift** flips to descending. Does not collide with Sort Lines Decimal Comma (`Cmd+Alt+F9`) or First Hidden Equal (`Cmd+Alt+F7`). Shortcut Mapper and About show the effective bindings.

## [0.3.337] — 2026-10-16

Edit:

- **Sort Lines Decimal Comma Asc / Desc** is remappable: Preferences → Sort Lines Decimal Comma shortcut, or `shortcut_sort_lines_decimal_comma` in `npp-rs/settings.json` (default `Cmd+Alt+F9`). Runs Edit → Line Operations → Sort Lines As Decimals (Comma) Ascending. **Shift** flips to descending. Does not collide with Find characters in range (`Alt+F9`) or Sort Lines Decimal Dot (`Cmd+Alt+F8`). Shortcut Mapper and About show the effective bindings.

## [0.3.336] — 2026-10-16

Edit:

- **Sort Lines Decimal Dot Asc / Desc** is remappable: Preferences → Sort Lines Decimal Dot shortcut, or `shortcut_sort_lines_decimal_dot` in `npp-rs/settings.json` (default `Cmd+Alt+F8`). Runs Edit → Line Operations → Sort Lines As Decimals (Dot) Ascending. **Shift** flips to descending. Does not collide with Next Change (`F8`), Find Purge (`Alt+Shift+F8`), Clear Change History (`Cmd+Shift+F8`), or Sort Lines Ignoring Case (`Cmd+Alt+F6`). Shortcut Mapper and About show the effective bindings.

## [0.3.335] — 2026-10-15

Edit:

- **Sort Lines Lex Ignoring Case Asc / Desc** is remappable: Preferences → Sort Lines Ignoring Case shortcut, or `shortcut_sort_lines_case_insensitive` in `npp-rs/settings.json` (default `Cmd+Alt+F6`). Runs Edit → Line Operations → Sort Lines Lex. Ascending Ignoring Case. **Shift** flips to descending ignoring case. Does not collide with Focus on Another View (`F6`), Find Cut (`Alt+Shift+F6`), or Sort Lines Integer (`Cmd+Alt+F5`). Shortcut Mapper and About show the effective bindings.

## [0.3.334] — 2026-10-15

Edit:

- **Sort Lines Integer Asc / Desc** is remappable: Preferences → Sort Lines Integer shortcut, or `shortcut_sort_lines_integer` in `npp-rs/settings.json` (default `Cmd+Alt+F5`). Runs Edit → Line Operations → Sort Lines As Integers Ascending. **Shift** flips to descending. Does not collide with Date Time (`F5`), Find Copy (`Alt+Shift+F5`), or Sort Lines Lexicographic (`Cmd+Alt+O`). Shortcut Mapper and About show the effective bindings.

## [0.3.333] — 2026-10-15

Search:

- **Find Next / Previous (Volatile)** uses the current selection as an ephemeral literal query and does **not** change the Find dialog / `find_query`. Honors Case / Word; ignores Re and Sel (whole document). Remappable via Preferences → Find Next (Volatile) shortcut, or `shortcut_find_volatile` in `npp-rs/settings.json` (default `Cmd+Alt+F3`; **Shift** = previous). Distinct from Select and Find (`Cmd+F3`), which copies the selection into the Find query. Does not collide with Find next (`F3`), Find Mark (`Alt+Shift+F3`), or Find All (`Cmd+Alt+F4`). Shortcut Mapper and About show the effective bindings.

## [0.3.332] — 2026-10-15

Search:

- **Find characters in range** is remappable: Preferences → Find characters in range shortcut, or `shortcut_find_char_range` in `npp-rs/settings.json` (default `Alt+F9`). Runs Search → Find characters in range (Find text = `ascii` / `non-ascii` / `start-end` such as `65-90`). **Shift** flips to the previous character in range. Chord parsing now accepts `F9` / `F10`. Does not collide with Find Purge (`Alt+Shift+F8` / `Alt+F8`), Next Change (`F8`), or Fullscreen (`F11`). Shortcut Mapper and About show the effective bindings.

## [0.3.331] — 2026-10-15

Search:

- **Find Purge** is remappable: Preferences → Find Purge shortcut, or `shortcut_find_purge` in `npp-rs/settings.json` (default `Alt+Shift+F8`). Opens Find, seeds from selection, deletes every line that contains a Find match with Case/Word/Re/Sel (same as Find-bar **Purge**; no clipboard). **Shift** flips to Purge matching substrings (Find-bar **Shift+Purge**). Does not collide with Next Hidden Equal (`Alt+F7` / `Alt+Shift+F7`), Next Change (`F8`), Clear Change History (`Cmd+Shift+F8`), or Find Cut (`Alt+Shift+F6`). Shortcut Mapper and About show the effective bindings.

## [0.3.330] — 2026-10-15

Search:

- **Find Cut** is remappable: Preferences → Find Cut shortcut, or `shortcut_find_cut` in `npp-rs/settings.json` (default `Alt+Shift+F6`). Opens Find, seeds from selection, cuts matching substrings to the clipboard and deletes them with Case/Word/Re/Sel (same as Find-bar **Cut**). **Shift** flips to Cut matching lines (Find-bar **Shift+Cut**). Does not collide with Focus on Another View (`F6`), Find Copy (`Alt+Shift+F5`), or Find Bookmark (`Alt+Shift+F4`). Shortcut Mapper and About show the effective bindings.

## [0.3.329] — 2026-10-15

Search:

- **Find Copy** is remappable: Preferences → Find Copy shortcut, or `shortcut_find_copy` in `npp-rs/settings.json` (default `Alt+Shift+F5`). Opens Find, seeds from selection, copies matching substrings with Case/Word/Re/Sel (same as Find-bar **Copy**). **Shift** flips to Copy matching lines (Find-bar **Lines**). Does not collide with Date Time (`F5`), Find Mark (`Alt+Shift+F3`), or Find Bookmark (`Alt+Shift+F4`). Shortcut Mapper and About show the effective bindings.

## [0.3.328] — 2026-10-15

Search:

- **Find Bookmark** is remappable: Preferences → Find Bookmark shortcut, or `shortcut_find_bookmark` in `npp-rs/settings.json` (default `Alt+Shift+F4`). Opens Find, seeds from selection, bookmarks all matching lines with Case/Word/Re/Sel (same as Find-bar **Bookmark**; keeps existing). Does not collide with Next Search Result (`F4`), Find All (`Cmd+Alt+F4`), Find Mark (`Alt+Shift+F3`), or Inverse Bookmarks (`Alt+Shift+F2`). Shortcut Mapper and About show the effective binding.

## [0.3.327] — 2026-10-15

Search:

- **Find Mark** is remappable: Preferences → Find Mark shortcut, or `shortcut_find_mark` in `npp-rs/settings.json` (default `Alt+Shift+F3`). Runs Search → Mark (opens Find, seeds from selection, Style 1 mark-all with Case/Word/Re/Sel; same as Find-bar **Mark**). Does not collide with Find next (`F3`), Select and Find (`Cmd+F3`), or Inverse Bookmarks (`Alt+Shift+F2`). Menu, Shortcut Mapper, and About show the effective binding.

## [0.3.326] — 2026-10-15

Search:

- **Mark current line Style 1…5** is remappable: Preferences → Mark current line style shortcut, or `shortcut_mark_one_style` in `npp-rs/settings.json` (default `Alt+Shift+1`). Digit-family chords accept `1`…`5` with those modifiers (mark the caret line with that style; Search → Mark → Using 1st…5th Style). Letter remaps mark Style 1 only. Does not collide with Mark Style (`Cmd+1`…`5`), Next Style (`Cmd+Alt+1`…`5`), Copy Styled (`Cmd+Alt+Shift+1`…`5`), or Fold Level (`Alt+1`…`8`). Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.325] — 2026-10-15

Search:

- **Copy Styled Text** is remappable: Preferences → Copy styled text shortcut, or `shortcut_style_to_clip` in `npp-rs/settings.json` (default `Cmd+Alt+Shift+1`). Digit-family chords accept `1`…`5` with those modifiers (copy that style's lines to the clipboard). Same mods + `0` copies all styled lines (Search → Copy All Styles to Clipboard). Letter remaps: base = Style 1; Shift = all. Does not collide with Mark Style (`Cmd+1`…`5`), Next Style (`Cmd+Alt+1`…`5`), or Fold Level (`Alt+1`…`8`). Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.324] — 2026-10-15

Search:

- **Clear Change History** is remappable: Preferences → Clear change history shortcut, or `shortcut_clear_change_history` in `npp-rs/settings.json` (default `Cmd+Shift+F8`). Clears amber/green change-history marks on the active tab (Search → Clear Change History). Stays near Next/Prev Change (`F8` / `Shift+F8`) without colliding (those chords have no Cmd). Menu, Shortcut Mapper, and About show the effective binding.

## [0.3.323] — 2026-10-15

Search:

- **Incremental Search** is remappable: Preferences → Incremental Search shortcut, or `shortcut_find_incremental` in `npp-rs/settings.json` (default `Alt+Shift+I`). Opens Find and enables as-you-type jump (same as Search → Incremental Search / Find-bar **Inc**). Does not collide with Indent Guide (`Alt+I`), Format Document (`Cmd+Shift+I`), or Close All to the Left (`Cmd+Alt+I`). Menu, Shortcut Mapper, and About show the effective binding.

## [0.3.322] — 2026-10-15

Search:

- **Remove Unmarked Lines** is remappable: Preferences → Remove unmarked lines shortcut, or `shortcut_remove_unmarked` in `npp-rs/settings.json` (default `Cmd+Alt+Z`). Runs Search → Remove Unmarked Lines (keeps bookmarked lines only). Does not collide with Word Wrap (`Alt+Z`), Zoom Sync (`Alt+Shift+Z`), or Paste Bookmarked (`Cmd+Alt+Q`). Menu, Shortcut Mapper, and About show the effective binding.

## [0.3.321] — 2026-10-15

Search:

- **Incremental Search**: Search → Incremental Search (or Find-bar **Inc**) enables as-you-type jump: each Find-field edit selects the first match at or after the selection start (growing the query lengthens in place). Closing Find clears Inc. Case / Word / Re / Sel / Wrap changes re-run while Inc is on.

## [0.3.320] — 2026-10-15

Search:

- **Find Results / Find in Files Shift+Enter**: with focus on a results tab (primary or dual-view secondary), **Shift+Enter** runs Previous Search Result (same as Shift+F4). Plain **Enter** still jumps the hit under the caret.

## [0.3.319] — 2026-10-15

Search:

- **Next / Previous Style marker** is remappable: Preferences → Next style marker shortcut, or `shortcut_next_style` in `npp-rs/settings.json` (default `Cmd+Alt+1`). Digit-family chords accept `1`…`5` with those modifiers (next that style). **Shift** flips to previous style. Does not collide with Mark Style (`Cmd+1`…`5`) or Fold Level (`Alt+1`…`8`). Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.318] — 2026-10-15

Search:

- **Replace bar Enter / Shift+Enter**: with focus in the Replace field, **Enter** runs Replace once (same as the Replace button); **Shift+Enter** runs Replace All. Hover tips on the field and buttons mention the chords.

## [0.3.317] — 2026-10-15

Search:

- **Find bar Shift+Enter**: with focus in the Find field, **Enter** still runs Find Next; **Shift+Enter** runs Find Previous (same as the Prev button). Hover tips on the field and Next/Prev mention the chords.

## [0.3.316] — 2026-10-15

Search:

- **Dual-view secondary pane** double-click on **Find Results** / **Find in Files** hit rows opens the source and parks/selects the match (same as primary-pane double-click and Enter). Works when the results tab is not the active primary tab.

## [0.3.315] — 2026-10-15

Search:

- **Enter-to-jump** on **Find Results** / **Find in Files**: with the caret on a hit row, Enter opens the source and parks/selects the match (same path as double-click). Header lines stay inert (read-only). Next/Prev Search Result (F4) unchanged.

## [0.3.314] — 2026-10-15

Search:

- **Find All / Search Results Window** is remappable: Preferences → Find All shortcut, or `shortcut_find_all` in `npp-rs/settings.json` (default `Cmd+Alt+F4`). Runs Search → Search Results Window (Find All current document). **Shift** flips to Find All in opened editor tabs. Does not collide with Next Search Result (`F4`) or Find in Files (`Cmd+Alt+F`). Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.313] — 2026-10-15

Search:

- **Click-to-jump** on **Find Results** / **Find in Files**: double-click a hit row (`line:col:match` or `path:line:text`) opens the source (from the workspace when needed) and parks/selects the match. Header lines still select a word on double-click. Next/Prev Search Result (F4) unchanged.

## [0.3.312] — 2026-10-15

Search:

- **Next / Previous Search Result** also walks the **Find in Files** tab: parks each `path:line:text` hit, opens the file from the workspace root when needed, selects the Find match on that line, and wraps with a status `· wrapped` bit. Find Results (Find All) walk is unchanged.

## [0.3.311] — 2026-10-15

Search:

- **Next / Previous Search Result** (F4 / Shift+F4, remappable `shortcut_next_found`) walks the **Find Results** tab from Find All / Shift+All: parks and selects each `line:col:match` in the source document (multi-doc sections too), wraps with a status `· wrapped` bit. Without a Find Results tab (or parseable hits), falls back to Find Next / Prev.

## [0.3.310] — 2026-10-15

Search:

- **Find All in opened documents** — Find bar **All** with **Shift** searches every open editor tab (skips Find Results / Find in Files) and refreshes the **Find Results** tab with per-document sections (Case / Word / Re; Sel ignored for multi-doc). Plain **All** stays current document only.

## [0.3.309] — 2026-10-15

Search:

- **Find All (current document)** — Find bar **All** opens (or refreshes) a read-only **Find Results** tab listing every match as `line:col:text` (Case / Word / Re / Sel; same engine as Find Next). Search → Search Results Window runs the same. Re-running while on the results tab searches the previous editor tab.

## [0.3.308] — 2026-10-15

Search:

- **Find Purge matching substrings** — Find bar **Purge** with **Shift** deletes every matching substring (Case / Word / Re / Sel; same engine as Find Next / Replace All) without touching the clipboard. Distinct from plain **Purge** (whole matching lines) and from **Cut** (clipboard + delete substrings).

## [0.3.307] — 2026-10-14

Search:

- **Find Cut matching lines** — Find bar **Cut** with **Shift** copies every line that contains a Find match to the clipboard and deletes those lines (Case / Word / Re / Sel; same engine as Find Next), newline-joined, deduped. Whole lines (body only), distinct from plain **Cut** (match substrings) and from **Purge** (delete lines without clipboard).

## [0.3.306] — 2026-10-14

Search:

- **Find Copy matching lines** — Find bar **Lines** copies every line that contains a Find match (Case / Word / Re / Sel; same engine as Find Next), newline-joined, deduped. Whole lines (body only), distinct from **Copy** (match substrings) and Copy Bookmarked Lines.

## [0.3.305] — 2026-10-14

Search:

- **Find Purge matching lines** — Find bar **Purge** deletes every line that contains a Find match (Case / Word / Re / Sel; same engine as Find Next). Whole lines, not substrings. Distinct from Cut (match text only) and Remove Bookmarked Lines.

## [0.3.304] — 2026-10-14

Search:

- **Find Cut matches** — Find bar **Cut** copies every matching substring to the clipboard and deletes those matches (Case / Word / Re / Sel; same engine as Find Next / Replace All), newline-joined. Distinct from Cut Bookmarked Lines (whole lines).

## [0.3.303] — 2026-10-14

Search:

- **Find Copy matches** — Find bar **Copy** copies every matching substring to the clipboard (Case / Word / Re / Sel; same engine as Find Next), newline-joined. Distinct from Copy Bookmarked Lines (whole lines).

## [0.3.302] — 2026-10-14

Search:

- **Find Bookmark all** — Find bar **Bookmark** bookmarks every matching line (Case / Word / Re / Sel; same engine as Find Next). Keeps existing bookmarks; status reports match lines and how many were new.

## [0.3.301] — 2026-10-14

Search:

- **Find Mark All** respects Case / Word / Re / Sel (same engine as Find Next). Find bar **Mark** styles matching lines with Style 1. Search → Mark runs the same when Find text is set. Style All Occurrences (Ctrl/Cmd+1…5) uses a selection token as a literal (Case/Word) or the Find query with full flags when nothing is selected.

## [0.3.300] — 2026-10-14

Settings / shortcuts:

- **Space to TAB (All)** is remappable: Preferences → Space to TAB (All) shortcut, or `shortcut_space_to_tab_all` in `npp-rs/settings.json` (default `Cmd+Alt+X`). Runs Edit → Space to TAB (All) (every four spaces → one tab). Does not collide with Expand Unchanged at Caret (`Alt+X`) or TAB to Space (`Cmd+Alt+V`). Menu, Shortcut Mapper, and About show the effective binding.

## [0.3.299] — 2026-10-14

Settings / shortcuts:

- **Paste / Remove Bookmarked Lines** is remappable: Preferences → Paste bookmarked lines shortcut, or `shortcut_paste_marked_lines` in `npp-rs/settings.json` (default `Cmd+Alt+Q`). Runs Search → Paste Bookmarked Lines. **Shift** flips to Remove Bookmarked Lines. Does not collide with Sync Vertical Scroll (`Alt+Q`), Paste (`Cmd+V`), or Copy Bookmarked Lines (`Cmd+Alt+F2`). Remove Unmarked stays menu-only. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.298] — 2026-10-14

Settings / shortcuts:

- **Copy / Cut Bookmarked Lines** is remappable: Preferences → Copy bookmarked lines shortcut, or `shortcut_copy_marked_lines` in `npp-rs/settings.json` (default `Cmd+Alt+F2`). Runs Search → Copy Bookmarked Lines. **Shift** flips to Cut Bookmarked Lines. Stays on the F2 bookmark family without colliding with next (`F2`), toggle (`Cmd+F2`), clear (`Cmd+Shift+F2`), or inverse (`Alt+Shift+F2`). Paste / Remove bookmarked lines stay menu-only. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.297] — 2026-10-14

Settings / shortcuts:

- **TAB to Space / Space to TAB (Leading)** is remappable: Preferences → TAB to Space shortcut, or `shortcut_tab_to_space` in `npp-rs/settings.json` (default `Cmd+Alt+V`). Runs Edit → TAB to Space (each tab → four spaces). **Shift** flips to Space to TAB (Leading). Space to TAB (All) stays menu-only. Does not collide with Paste (`Cmd+V`), Show White Space (`Alt+V`), or Trim Trailing (`Cmd+Alt+T`). Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.296] — 2026-10-14

Settings / shortcuts:

- **First / Last Tab** is remappable: Preferences → First tab shortcut, or `shortcut_first_tab` in `npp-rs/settings.json` (default `Cmd+Alt+Home`). Runs View → First Tab. Opposite **End** jumps to Last Tab; letter remaps use **Shift** for last. Does not collide with Document Home (`Cmd+Home`) or Next tab (`Cmd+Tab`). Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.295] — 2026-10-14

Settings / shortcuts:

- **Next / Previous Search Result** is remappable: Preferences → Next search result shortcut, or `shortcut_next_found` in `npp-rs/settings.json` (default `F4`). Runs Search → Next Search Result. **Shift** flips to Previous Search Result. Chord parser accepts `F4`. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.294] — 2026-10-14

Settings / shortcuts:

- **Path autocomplete** is remappable: Preferences → Path autocomplete shortcut, or `shortcut_autocomplete_path` in `npp-rs/settings.json` (default `Cmd+Alt+Space`). Runs Edit → Function Completion Path. Menu, Shortcut Mapper, and About show the effective binding.

## [0.3.293] — 2026-10-14

Settings / shortcuts:

- **Autocomplete / Function Call Tip** is remappable: Preferences → Autocomplete shortcut, or `shortcut_autocomplete` in `npp-rs/settings.json` (default `Cmd+Space`). Runs Edit → Function Completion File. **Shift** flips to Function Call Tip. Path completion stayed menu-only until v0.3.294. Menu, Shortcut Mapper, and About show the effective binding.

## [0.3.292] — 2026-10-14

Settings / shortcuts:

- **Fold level 1…8** is remappable: Preferences → Fold level shortcut, or `shortcut_fold_level` in `npp-rs/settings.json` (default `Alt+1`). Digit-family chords accept `1`…`8` with those modifiers (fold that indent level). **Shift** flips to Unfold Level. `Alt+0` stays Fold All. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.291] — 2026-10-14

Settings / shortcuts:

- **Rect column select** is remappable: Preferences → Rect column select shortcut, or `shortcut_rect_select` in `npp-rs/settings.json` (default `Alt+Shift+Left`). Arrow-family chords accept any arrow with those modifiers (Left/Right grow columns; Up/Down grow rows). Letter remaps: base = left, Shift-flipped partner = right. On the default chord, Alt+Shift+arrows still prefer rect over word-jump extend. Shortcut Mapper and About show the effective binding.

## [0.3.290] — 2026-10-14

Settings / shortcuts:

- **Zoom Sync** is remappable: Preferences → Zoom Sync shortcut, or `shortcut_zoom_sync` in `npp-rs/settings.json` (default `Alt+Shift+Z`). Toggles dual-view shared font size. Menu, Shortcut Mapper, and About show the effective binding. Stays near word wrap (`Alt+Z`); Sync H keeps `Alt+Shift+Q`.

## [0.3.289] — 2026-10-14

Settings / shortcuts:

- **Inverse Bookmarks** is remappable: Preferences → Inverse bookmarks shortcut, or `shortcut_inverse_bookmarks` in `npp-rs/settings.json` (default `Alt+Shift+F2`). Inverts bookmarks on every line in the active tab. Menu, Shortcut Mapper, and About show the effective binding. Stays on the F2 family without colliding with next/prev/toggle/clear.

## [0.3.288] — 2026-10-14

Settings / shortcuts:

- **Clear all styles** is remappable: Preferences → Clear all styles shortcut, or `shortcut_clear_all_styles` in `npp-rs/settings.json` (default `Cmd+Shift+0`). Clears every style mark (1–5) in the active tab. Menu, Shortcut Mapper, and About show the effective binding. `Cmd+0` stays zoom restore; `Alt+0` stays fold all.

## [0.3.287] — 2026-10-14

Settings / shortcuts:

- **Mark Style 5** is remappable: Preferences → Mark Style 5 shortcut, or `shortcut_mark_style_5` in `npp-rs/settings.json` (default `Cmd+5`). Marks all Find/selection matches with Style 5. **Shift** flips to Clear Style 5. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.286] — 2026-10-14

Settings / shortcuts:

- **Mark Style 4** is remappable: Preferences → Mark Style 4 shortcut, or `shortcut_mark_style_4` in `npp-rs/settings.json` (default `Cmd+4`). Marks all Find/selection matches with Style 4. **Shift** flips to Clear Style 4. Style 5 stays menu-only. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.285] — 2026-10-14

Settings / shortcuts:

- **Mark Style 3** is remappable: Preferences → Mark Style 3 shortcut, or `shortcut_mark_style_3` in `npp-rs/settings.json` (default `Cmd+3`). Marks all Find/selection matches with Style 3. **Shift** flips to Clear Style 3. Styles 4–5 stay menu-only. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.284] — 2026-10-14

Settings / shortcuts:

- **Mark Style 2** is remappable: Preferences → Mark Style 2 shortcut, or `shortcut_mark_style_2` in `npp-rs/settings.json` (default `Cmd+2`). Marks all Find/selection matches with Style 2. **Shift** flips to Clear Style 2. Styles 3–5 stay menu-only. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.283] — 2026-10-14

Settings / shortcuts:

- **Mark Style 1** is remappable: Preferences → Mark Style 1 shortcut, or `shortcut_mark_style` in `npp-rs/settings.json` (default `Cmd+1`). Marks all Find/selection matches with Style 1. **Shift** flips to Clear Style 1. Styles 2–5 stay menu-only. Menu, Shortcut Mapper, and About show the effective bindings. Chord parser now accepts digit keys `1`–`5`.

## [0.3.282] — 2026-10-14

Settings / shortcuts:

- **Select and Find Next** is remappable: Preferences → Select and Find shortcut, or `shortcut_set_and_find` in `npp-rs/settings.json` (default `Cmd+F3`). Copies the selection into the find query then finds next. **Shift** flips to Select and Find Previous. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.281] — 2026-10-14

Settings / shortcuts:

- **Move Current Tab Forward** is remappable: Preferences → Move tab shortcut, or `shortcut_move_tab` in `npp-rs/settings.json` (default `Cmd+Shift+PageDown`). Opposite PageUp moves the tab backward; letter remaps use Shift for backward. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.280] — 2026-10-13

Settings / shortcuts:

- **Function List** is remappable: Preferences → Function List shortcut, or `shortcut_function_list` in `npp-rs/settings.json` (default `Cmd+E`). **Shift** flips to Character Panel. Both panels toggle open/closed. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.279] — 2026-10-13

Settings / shortcuts:

- **Save a Copy As** is remappable: Preferences → Save a Copy As shortcut, or `shortcut_save_copy` in `npp-rs/settings.json` (default `Cmd+Alt+Y`). **Shift** flips to Rename. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.278] — 2026-10-13

Settings / shortcuts:

- **Close All to the Left** is remappable: Preferences → Close All to Left shortcut, or `shortcut_close_to_side` in `npp-rs/settings.json` (default `Cmd+Alt+I`). **Shift** flips to Close All to the Right. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.277] — 2026-10-13

Settings / shortcuts:

- **Close All but Pinned** is remappable: Preferences → Close All but Pinned shortcut, or `shortcut_close_but_pinned` in `npp-rs/settings.json` (default `Cmd+Alt+H`). **Shift** flips to Close All Unchanged. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.276] — 2026-10-13

Settings / shortcuts:

- **Open Folder as Workspace** is remappable: Preferences → Open workspace shortcut, or `shortcut_open_workspace` in `npp-rs/settings.json` (default `Cmd+Alt+G`). **Shift** flips to Open Containing Folder as Workspace. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.275] — 2026-10-13

Settings / shortcuts:

- **Open Containing Folder** is remappable: Preferences → Open folder shortcut, or `shortcut_open_folder` in `npp-rs/settings.json` (default `Cmd+Alt+L`). **Shift** flips to open a shell in the active file's directory. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.274] — 2026-10-13

Settings / shortcuts:

- **Open Selected Filename** is remappable: Preferences → Open selected shortcut, or `shortcut_open_selected` in `npp-rs/settings.json` (default `Cmd+Alt+J`). **Shift** flips to Open Containing Folder in the file manager. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.273] — 2026-10-13

Settings / shortcuts:

- **Copy All Paths** is remappable: Preferences → Copy all paths shortcut, or `shortcut_copy_all_paths` in `npp-rs/settings.json` (default `Cmd+Alt+M`). **Shift** flips to Copy All Filenames. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.272] — 2026-10-13

Settings / shortcuts:

- **Copy Current Directory Path** is remappable: Preferences → Copy directory shortcut, or `shortcut_copy_dir` in `npp-rs/settings.json` (default `Cmd+Alt+K`). Menu, Shortcut Mapper, and About show the effective binding.

## [0.3.271] — 2026-10-13

Settings / shortcuts:

- **Copy Full Path** is remappable: Preferences → Copy path shortcut, or `shortcut_copy_path` in `npp-rs/settings.json` (default `Cmd+Alt+P`). **Shift** flips to Copy File Name. Current directory stayed menu-only until v0.3.272. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.270] — 2026-10-13

Settings / shortcuts:

- **Date Time (short)** is remappable: Preferences → Date Time shortcut, or `shortcut_insert_datetime` in `npp-rs/settings.json` (default `F5`). **Shift** flips to Date Time (long). Customized format stays menu-only. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.269] — 2026-10-13

Settings / shortcuts:

- **Remove Consecutive Duplicate Lines** is remappable: Preferences → Remove Duplicate Lines shortcut, or `shortcut_remove_dup_lines` in `npp-rs/settings.json` (default `Cmd+Alt+D`). **Shift** flips to Remove Duplicate Lines (any occurrence). Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.268] — 2026-10-13

Settings / shortcuts:

- **Remove Empty Lines** is remappable: Preferences → Remove Empty Lines shortcut, or `shortcut_remove_empty_lines` in `npp-rs/settings.json` (default `Cmd+Alt+E`). **Shift** flips to Remove Empty Lines (Containing Blank Characters). Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.267] — 2026-10-13

Settings / shortcuts:

- **Sort Lines Lexicographic Ascending** is remappable: Preferences → Sort Lines shortcut, or `shortcut_sort_lines` in `npp-rs/settings.json` (default `Cmd+Alt+O`). **Shift** flips to Lexicographic Descending. Menu, Shortcut Mapper, and About show the effective bindings.

## [0.3.266] — 2026-10-13

Settings / shortcuts:

- **Column Editor** is remappable: Preferences → Column Editor shortcut, or `shortcut_column_editor` in `npp-rs/settings.json` (default `Cmd+Alt+C`). Menu, Shortcut Mapper, and About show the effective binding.

## [0.3.265] — 2026-10-13

Settings / shortcuts:

- **Toggle Read-Only** is remappable: Preferences → Toggle read-only shortcut, or `shortcut_toggle_readonly` in `npp-rs/settings.json` (default `Cmd+Alt+R`). Menu, Shortcut Mapper, and About show the effective binding.

## [0.3.264] — 2026-10-13

Edit / column mode:

- **Document Home/End** (Cmd/Ctrl+Home/End and Cmd/Ctrl+Up/Down) move multi-carets / rect blocks to the document edge; **Shift** extends stream multi-carets (keep anchors) and grows rect lines to the first/last document line (`docs/column-mode.md`).

## [0.3.263] — 2026-10-13

Edit / column mode:

- **Alt+Shift+arrows** start or extend a rectangular column selection from the caret (N++-style keyboard column select), including virtual columns past EOL. Prefer Up/Down first for a multi-line block; Left/Right then grow columns. On the default Alt+Left/Right word-jump chord, Alt+Shift+Left/Right prefers rect over word-jump extend.

## [0.3.262] — 2026-10-13

Edit / column mode:

- **Shift+Home / Shift+End** extend rectangular selections (keep Alt+drag anchor corner; Home → column 0, End → that line’s body end), same idea as Shift+Left/Right on a rect.

## [0.3.261] — 2026-10-13

Edit / column mode:

- **Begin/End Select in Column Mode** keeps virtual columns past EOL on both marks (free-caret space), so the rect matches Alt+drag past EOL. Stream Begin/End Select can also end in virtual space.

## [0.3.260] — 2026-10-13

Edit / column mode / shortcuts:

- **Begin/End Select in Column Mode** builds a real rectangular selection between the marked start caret and the current caret (same corners as Alt+drag). Stream Begin/End Select is unchanged.
- **Begin/End Select** is remappable: Preferences → Begin/End Select shortcut, or `shortcut_begin_end_select` in `npp-rs/settings.json` (default `Cmd+Alt+B`). Shift flips to **Begin/End Select in Column Mode**. Menu, Shortcut Mapper, and About show both bindings.

## [0.3.259] — 2026-10-13

Settings / shortcuts:

- **Multi-select All** is remappable: Preferences → Multi-select all shortcut, or `shortcut_multi_select_all` in `npp-rs/settings.json` (default `Cmd+Alt+A`). Shift flips to **Multi-select All (Whole Word)**. Menu, Shortcut Mapper, and About show both bindings.

## [0.3.258] — 2026-10-13

Edit / column mode:

- **Multi-caret / rect paste line-split**: when the clipboard has the same number of lines as carets (typical after a multi-caret or column copy), each caret gets its matching line. Mismatched line counts still paste the full string at every caret (`docs/column-mode.md`).

## [0.3.257] — 2026-10-13

Settings / shortcuts:

- **Trim Trailing Space** is remappable: Preferences → Trim Trailing shortcut, or `shortcut_trim_trailing` in `npp-rs/settings.json` (default `Cmd+Alt+T`). Shift flips to **Trim Leading and Trailing Space**. Menu, Shortcut Mapper, and About show both bindings.

## [0.3.256] — 2026-10-13

Settings / shortcuts:

- **Close All** is remappable: Preferences → Close All shortcut, or `shortcut_close_all` in `npp-rs/settings.json` (default `Cmd+Alt+W`). Shift flips to **Close All but Current**. Menu, Shortcut Mapper, and About show both bindings.

## [0.3.255] — 2026-10-13

Edit / column mode:

- **Escape clears multi-carets**: remappable close-Find chord (default Escape) closes Find/Replace when open; when closed, clears multi-carets / column select (`docs/column-mode.md`).

## [0.3.254] — 2026-10-12

Edit / column mode:

- **Multi-caret Page Up/Down**: remappable Page Up/Down (default PageUp/PageDown) moves every multi-caret / rect block by a page (±30 lines); Shift extends stream multi-carets (and rect Shift+extend). Same path as plain Up/Down, larger delta (`docs/column-mode.md`).

## [0.3.253] — 2026-10-12

Edit / column mode:

- **Multi-caret line Home/End**: remappable line Home/End (default Home/End) moves every multi-caret to that line’s start or body end; Shift extends stream multi-carets. Rect selections become stream multi after a plain Home/End (`docs/column-mode.md`).

## [0.3.252] — 2026-10-12

Edit / column mode:

- **Multi-caret word-jump**: remappable word-jump (default Alt+Left/Right) moves every multi-caret together; Shift extends stream multi-carets. Rect selections become stream multi after a plain word-jump (`docs/column-mode.md`).

## [0.3.251] — 2026-10-12

Edit / column mode:

- **Rect Shift+extend**: Shift+Left/Right/Up/Down grow or shrink an Alt+drag rectangular selection (anchor corner kept, caret corner moves, including past EOL). Collapsing to one line becomes a stream selection (`docs/column-mode.md`).

## [0.3.250] — 2026-10-12

Edit / column mode:

- **Shift+extend multi-carets**: Shift+Left/Right/Up/Down extend every stream multi-caret selection (anchor kept, caret moves). Typing replaces each extended range. Rect Alt+drag still clears on Shift (`docs/column-mode.md`).

## [0.3.249] — 2026-10-12

Edit / column mode:

- **Ctrl/Cmd+click add carets**: click with Ctrl (Cmd on macOS) adds a zero-width caret; click an existing caret again to remove it. Seeds the previous caret/selection into the multi-caret set.

## [0.3.248] — 2026-10-12

Edit / column mode:

- **Multi-caret arrow move**: plain Left/Right/Up/Down move every multi-caret / rect caret together (ranges collapse to an edge first; rect keeps virtual columns). Shift / word-jump / document jumps still clear extras (`docs/column-mode.md`).

## [0.3.247] — 2026-10-12

Edit / column mode:

- **Stream select into virtual space**: Shift+click / drag (and plain drag) past EOL extends the selection wash and caret; Shift+Right / Shift+Left grow or shrink virtual space at EOL. Buffer selection still clamps to real characters (`docs/column-mode.md`).

## [0.3.246] — 2026-10-12

Edit / column mode:

- **Plain free caret past EOL**: click or Right past the line end places a virtual caret; paint follows; typing / paste / Tab pad with spaces; Backspace / Left shrink virtual space; Up/Down keep the preferred column (`docs/column-mode.md`).

## [0.3.245] — 2026-10-12

Edit / column mode:

- **Alt+drag past EOL**: mouse hit-test can place rectangular carets / selection past every line’s end; carets and the wash draw in that virtual space; typing still pads with spaces (`docs/column-mode.md`).

## [0.3.244] — 2026-10-12

Edit / column mode:

- **Virtual space** for Alt+drag rectangular selections: typing on short lines pads with spaces to the target column; copy includes space padding for columns past EOL. Carets still clamp visually until padded (`docs/column-mode.md`).

## [0.3.243] — 2026-10-12

Docs (issue #20):

- Refresh **`docs/next-gaps.md`**: drop the stale post-v0.3.12 / open-#14 table; document what shipped through v0.3.242 and rank real remaining gaps (shortcuts.xml, column virtual space, Find/FIF UI depth, LSP, macros, plugins ABI, UDL, hex, 3-way, …). Point overnight idle work at `docs/autoresearch/standing_backlog.md`.
- Bump `docs/whats-missing.md` / docs index checkpoint dates to match.

## [0.3.242] — 2026-10-12

Plugins / Ollama (issue #19):

- **Ask Ollama** and **Ollama Status** run on a background thread so long generations no longer freeze the window. Status bar shows progress; reply / status still open in a tab when ready. One request at a time.
- Editor **right-click → Ask Ollama** (same as Plugins → Ask Ollama).

## [0.3.241] — 2026-10-12

View / Preferences:

- **Post-It / Distraction Free** hides the menu and status bar (tabs and editor stay). No longer an alias for OS fullscreen (F11 stays fullscreen). State persists in `npp-rs/settings.json` (`post_it`). Menu ✓, remappable F12 (`shortcut_postit`), and Preferences checkbox restore on launch. Exit with the same chord when chrome is hidden.

## [0.3.240] — 2026-10-12

View / Preferences / Dual view:

- **Zoom Sync** persists in `npp-rs/settings.json` (`zoom_sync`). Menu toggle, Preferences checkbox, and relaunch restore shared font size across panes.
- **Sync Vertical / Horizontal / Zoom Sync** menu and remappable toggles now flip the live dual-view flags (no process-wide atomics). After session restore of `@syncv` / `@synch`, or Clear Compare restoring pre-start sync, the next click turns sync off correctly.

## [0.3.239] — 2026-10-12

File / Session / Compare:

- Opt-in session restore now keeps a **marked Compare partner** (`@partner`, alias `@cmppartner`) in `npp-rs/session.txt` when a tab was ⌘/Ctrl-clicked (or marked via the tab context menu) for Compare. Relaunch re-marks that tab so Compare with Other View still prefers it. Older session files still load (no partner mark by default).

## [0.3.238] — 2026-10-12

View / Preferences:

- **Project panel** open/closed persists in `npp-rs/settings.json` (`show_project_panel`). Menu / remappable toggle, window close, and Preferences checkbox restore the panel on launch.

## [0.3.237] — 2026-10-12

File / Session / Compare:

- Opt-in session restore now keeps **Compare on** (`@compare`, alias `@cmp`) in `npp-rs/session.txt` with dual-view `@other`. Relaunch re-opens the split and restarts Compare (parks the first hunk). Older session files still load (Compare off by default).

## [0.3.236] — 2026-10-12

View / Preferences:

- **Document Map** open/closed persists in `npp-rs/settings.json` (`show_document_map`). Menu / remappable toggle, window close, and Preferences checkbox restore the panel on launch.

## [0.3.235] — 2026-10-12

View / Preferences:

- **Always on Top** persists in `npp-rs/settings.json` (`always_on_top`). Menu / Alt+T toggle and Preferences checkbox restore the window level on launch (same pattern as `text_rtl`).

## [0.3.234] — 2026-10-12

File / Session:

- Opt-in session restore now keeps **Synchronize Vertical / Horizontal Scrolling** (`@syncv` / `@synch`, aliases `@syncvert` / `@synchoriz`) in `npp-rs/session.txt` with dual-view `@other`. Relaunch re-opens the split with the same sync flags. Older session files still load (sync off by default).

## [0.3.233] — 2026-10-12

File / Session:

- Opt-in session restore now keeps **dual-view other-pane focus** (`@focus`, alias `@otherfocus`) in `npp-rs/session.txt` with `@other`. Relaunch re-opens the split and puts keyboard focus on the other pane when that is how you left it. Older session files still load (primary focus remains the default).

## [0.3.232] — 2026-10-12

File / Session:

- Opt-in session restore now keeps **dual-view other pane** (`@other`, alias `@otherview`) in `npp-rs/session.txt`, in addition to paths, `@folds`, `@bookmarks`, `@style1`…`@style5`, `@caret`, `@sel`, `@scroll`, `@lang`, `@encoding`, `@readonly`, `@pin`, `@colour`, `@tail`, and `@active`. Move / Clone to Other View layout survives relaunch (including async opens). Older session files still load.

## [0.3.231] — 2026-10-12

File / Session:

- Opt-in session restore now keeps **Search → Style marks** (`@style1`…`@style5` line lists) in `npp-rs/session.txt`, in addition to paths, `@folds`, `@bookmarks`, `@caret`, `@sel`, `@scroll`, `@lang`, `@encoding`, `@readonly`, `@pin`, `@colour`, `@tail`, and `@active`. Colored style marks survive relaunch (including async opens). Older session files still load.

## [0.3.230] — 2026-10-12

File / Session:

- Opt-in session restore now keeps **selection** (`@sel A_LINE A_COL C_LINE C_COL`, anchor then caret) in `npp-rs/session.txt`, in addition to paths, `@folds`, `@bookmarks`, `@caret`, `@scroll`, `@lang`, `@encoding`, `@readonly`, `@pin`, `@colour`, `@tail`, and `@active`. Highlighted ranges survive relaunch (including async opens; `@tail` still clears selection). Older session files still load.

## [0.3.229] — 2026-10-12

File / Session:

- Opt-in session restore now keeps **log-tail follow** (`@tail`, alias `@logtail`) in `npp-rs/session.txt`, in addition to paths, `@folds`, `@bookmarks`, `@caret`, `@scroll`, `@lang`, `@encoding`, `@readonly`, `@pin`, `@colour`, and `@active`. View → Monitoring (tail) survives relaunch (including async opens; skips the Ask prompt when `@tail` is present). Older session files still load.

## [0.3.228] — 2026-10-12

File / Session:

- Opt-in session restore now keeps **tab colour** (`@colour N`, alias `@color N`, 1..=5) in `npp-rs/session.txt`, in addition to paths, `@folds`, `@bookmarks`, `@caret`, `@scroll`, `@lang`, `@encoding`, `@readonly`, `@pin`, and `@active`. View → Tab colour survives relaunch. Older session files still load.

## [0.3.227] — 2026-10-12

File / Session:

- Opt-in session restore now keeps **per-tab** first visible line (`@scroll LINE`) for every open path in `npp-rs/session.txt` (not only `@active`). Tab clicks and next/prev tab restore each tab’s viewport instead of jumping to line 0. `@scroll` also survives sync open without being wiped by the post-open view reset.

## [0.3.226] — 2026-10-12

File / Session:

- Opt-in session restore now keeps the **first visible line** (`@scroll LINE`) for the active tab in `npp-rs/session.txt`, in addition to paths, `@folds`, `@bookmarks`, `@caret`, `@lang`, `@encoding`, `@readonly`, `@pin`, and `@active`. Viewport scroll survives relaunch (wins over caret follow when present; including async opens). Older session files still load (no `@scroll` → scroll follows restored caret as before).

## [0.3.225] — 2026-10-11

File / Session:

- Opt-in session restore now keeps **read-only** (`@readonly`, alias `@ro`) in `npp-rs/session.txt`, in addition to paths, `@folds`, `@bookmarks`, `@caret`, `@lang`, `@encoding`, `@pin`, and `@active`. Edit → Toggle Read-Only survives relaunch (including async opens). Older session files still load.

## [0.3.224] — 2026-10-11

File / Session:

- Opt-in session restore now keeps **save encoding** (`@encoding TOKEN`) in `npp-rs/session.txt`, in addition to paths, `@folds`, `@bookmarks`, `@caret`, `@lang`, `@pin`, and `@active`. Overrides open detect after load (including async opens) so Format → Convert / ANSI vs UTF-8 choices survive relaunch. Tokens: `utf8`, `utf8-bom`, `utf16-le`, `utf16-be`, `windows-1252` (aliases accepted). Older session files still load.

## [0.3.223] — 2026-10-11

File / Session:

- Opt-in session restore now keeps **Language menu** highlight id (`@lang ID`) in `npp-rs/session.txt`, in addition to paths, `@folds`, `@bookmarks`, `@caret`, `@pin`, and `@active`. Overrides path-based detect after open (including async loads). Older session files still load.

## [0.3.222] — 2026-10-11

File / Session:

- Opt-in session restore now keeps **pinned tabs** (`@pin`) in `npp-rs/session.txt`, in addition to paths, `@folds`, `@bookmarks`, `@caret`, and `@active`. Older session files still load.

## [0.3.221] — 2026-10-11

File / Session:

- Opt-in session restore now keeps **bookmarks** (`@bookmarks l1,l2,…`) in `npp-rs/session.txt`, in addition to paths, `@folds`, `@caret`, and `@active`. Lines past EOF are skipped. Older session files still load.

## [0.3.220] — 2026-10-11

View / Settings:

- **Show Fold Margin** is remappable (`shortcut_show_fold_margin`, default Alt+G). View → Show Symbol → Show Fold Margin, Preferences, Shortcut Mapper, and About show the effective binding. Persists via `show_fold_margin` as before.

## [0.3.219] — 2026-10-11

File / Session:

- Opt-in session restore now keeps **caret line/column** (`@caret LINE COL`) and the **active tab** (`@active`) in `npp-rs/session.txt`, in addition to paths and `@folds`. Scroll follows the restored caret on launch. Older path-only / folds-only session files still load.

## [0.3.218] — 2026-10-11

View / Settings:

- **Show Line Numbers** is remappable (`shortcut_show_line_numbers`, default Alt+L). View → Show Symbol → Show Line Numbers, Preferences, Shortcut Mapper, and About show the effective binding. Persists via `show_line_numbers` as before.

## [0.3.217] — 2026-10-11

Edit / Settings:

- **RTL / LTR** is remappable (`shortcut_rtl`, default Alt+R). Shift + that chord sets LTR. Menu, Preferences, Shortcut Mapper, and About show the effective binding. Persists via `text_rtl` as before.

## [0.3.216] — 2026-10-11

View / Settings:

- **Move / Clone to Other View** is remappable (`shortcut_move_other_view`, default Alt+O). Shift + that chord clones the document to the other view. Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.215] — 2026-10-11

View:

- **Dual-view other pane** paints Tree-sitter syntax colours (same viewport window as the primary pane). Edits on that tab refresh its own highlight cache; tab close / reorder remaps the cache.

## [0.3.214] — 2026-10-11

View / Settings:

- **Post-It** is remappable (`shortcut_postit`, default F12). Menu, Preferences, Shortcut Mapper, and About show the effective binding. Same distraction-free toggle as the View → Post-It command (viewport fullscreen).

## [0.3.213] — 2026-10-11

View / Settings:

- **Show Non-Printing Characters** is remappable (`shortcut_show_npc`, default Alt+N). Menu, Preferences, Shortcut Mapper, and About show the effective binding. Show All Characters (Alt+A) still toggles whitespace + EOL + NPC together.

## [0.3.212] — 2026-10-11

View / Settings:

- **Synchronize Vertical / Horizontal Scrolling** is remappable (`shortcut_sync_scroll`, default Alt+Q). Shift + that chord toggles **Synchronize Horizontal Scrolling**. Menu, Preferences, Shortcut Mapper, and About show the effective binding. Either toggle opens dual view.

## [0.3.211] — 2026-10-11

View / Settings:

- **Show White Space and TAB** is remappable (`shortcut_show_whitespace`, default Alt+V). Shift + that chord toggles **Show EOL**. Menu, Preferences, Shortcut Mapper, and About show the effective binding. Show All Characters (Alt+A) still toggles whitespace + EOL + NPC together.

## [0.3.210] — 2026-10-11

File / Settings:

- **Pin tab** is remappable (`shortcut_pin_tab`, default Alt+Shift+P). Toggles the active tab pin (`IDM_PINTAB`). Tab chrome, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.209] — 2026-10-11

View / Settings:

- **Project Panel** toggles open/closed (was open-only) and is remappable (`shortcut_project_panel`, default Alt+P). Menu, Preferences, Shortcut Mapper, and About show the effective binding. File → Open Folder as Workspace still force-opens the panel.

## [0.3.208] — 2026-10-11

View / Settings:

- **Indent Guide** is remappable (`shortcut_indent_guide`, default Alt+I). Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.207] — 2026-10-11

View / Settings:

- **Show All Characters** is remappable (`shortcut_show_all_characters`, default Alt+A). Menu, Preferences, Shortcut Mapper, and About show the effective binding. Toggles whitespace + EOL + NPC together (same as the View menu command).

## [0.3.206] — 2026-10-11

Search / Edit:

- **Drop selection into Replace**: with Replace open, drag selected text onto the Replace field to fill the replacement text (first line, max 200 chars). Source text is not moved. Orange highlight on the Replace field while hovering.

## [0.3.205] — 2026-10-11

Search / Edit:

- **Drop selection into Find**: with the Find bar open, drag selected text onto the Find field to fill the query (first line, max 200 chars). Source text is not moved. Orange highlight on the Find field while hovering.

## [0.3.204] — 2026-10-11

Edit / View:

- **Selection drag across dual view**: drag selected text from one pane into the other to move it (Ctrl/Cmd = copy). Orange drop caret follows the destination pane. Same-buffer drag is unchanged.

## [0.3.203] — 2026-10-11

View / Settings:

- **Always on Top** toggles on/off (was set-only) and is remappable (`shortcut_always_on_top`, default Alt+T). Menu, Preferences, Shortcut Mapper, and About show the effective binding. Keyboard fullscreen toggle also applies the viewport command (same path as menu).

## [0.3.202] — 2026-10-11

View / Settings:

- **Fullscreen** is remappable (`shortcut_fullscreen`, default F11). Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.201] — 2026-10-11

Edit / Settings:

- **Multi-select Next / Skip** is remappable (`shortcut_multi_select_next`, default Cmd+Alt+N). Shift + that chord skips the current match and jumps to the next. Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.200] — 2026-10-11

Edit / Settings:

- **Enter / newline** is remappable (`shortcut_enter`, default Enter). When remapped, bare Enter no longer inserts a newline; the chord inserts preferred EOL. Blank line (Cmd+Enter) stays separate. Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.199] — 2026-10-11

Edit / Settings:

- **RTL / LTR text direction** persists in `npp-rs/settings.json` (`text_rtl`). Edit → RTL / LTR and Preferences → Right-to-left text restore on launch (line anchors + status cue). Full bidi / UI chrome mirror still open.

## [0.3.198] — 2026-10-10

View / Compare:

- **Preferences Compare toggles** (ignore whitespace / case / blank, hide unchanged) match the View menu while Compare is on: immediate re-diff or hide apply, first-hunk park + select, and pair status with `· at L|R …` (no more delayed stale re-diff / generic “Preferences saved” only).

## [0.3.197] — 2026-10-10

View / Compare:

- **Clear Compare** restores the dual-pane sync H/V scroll flags from before Start Compare forced them on (so leaving Compare does not leave scroll stuck synced).

## [0.3.196] — 2026-10-10

Edit / Settings:

- **Document Home / End aliases** are remappable (`shortcut_document_home`, default Cmd+Home). Opposite End jumps to document end; letter remaps use Shift for end. When remapped, bare Cmd+Home/End no longer jump. Document start (Cmd+Up) stays separate. Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.195] — 2026-10-10

Edit:

- **Shift+Up / Down** and **Shift+PageUp / PageDown** extend the selection while moving (same as Shift+Left/Right and Shift+Home/End). Letter remaps that use Shift for the opposite direction keep that meaning and do not also extend.

## [0.3.194] — 2026-10-10

Edit / Settings:

- **Line up / down** is remappable (`shortcut_line_up`, default Up). Opposite Down jumps one line down; letter remaps use Shift for down. When remapped, bare Up/Down no longer move by line. Move line and document start/end stay separate. Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.193] — 2026-10-10

Edit / Settings:

- **Character left / right** is remappable (`shortcut_char_left`, default Left). Opposite Right jumps one character right; letter remaps use Shift for right. When remapped, bare Left/Right no longer move by character. Word jump stays separate. Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.192] — 2026-10-10

Edit / Settings:

- **Page up / down** is remappable (`shortcut_page_up`, default PageUp). Opposite PageDown jumps page down; letter remaps use Shift for page down. When remapped, bare PageUp/PageDown no longer page. Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.191] — 2026-10-10

Edit / Settings:

- **Line start / end** is remappable (`shortcut_line_home`, default Home). Opposite End jumps to line end; letter remaps use Shift for end. When remapped, bare Home/End no longer move on the line. `Cmd+Home` / `Cmd+End` stay hard-wired document start/end aliases. Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.190] — 2026-10-10

Edit / Settings:

- **Backspace** (delete backward) is remappable (`shortcut_delete_backward`, default Backspace). When remapped, bare Backspace no longer deletes backward; the remapped chord deletes backward (same as editor Backspace). Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.189] — 2026-10-10

Edit / Settings:

- **Delete** (forward) is remappable (`shortcut_delete_forward`, default Delete). When remapped, bare Delete no longer deletes forward; the remapped chord runs Edit → Delete. Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.188] — 2026-10-10

Edit / Settings:

- **Paste** is remappable (`shortcut_paste`, default Cmd+V). When remapped, platform Cmd+V no longer pastes; the remapped chord pastes last copied text (same as Edit → Paste). Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.187] — 2026-10-10

Edit / Settings:

- **Cut** is remappable (`shortcut_cut`, default Cmd+X). When remapped, platform Cmd+X no longer cuts. Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.186] — 2026-10-10

Edit / Settings:

- **Copy** is remappable (`shortcut_copy`, default Cmd+C). When remapped, platform Cmd+C no longer copies. Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.185] — 2026-10-10

Search / Settings:

- **Clear all bookmarks** is remappable (`shortcut_clear_bookmarks`, default Cmd+Shift+F2). Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.184] — 2026-10-10

Edit / Settings:

- **Blank line below / above** is remappable (`shortcut_blank_line`, default Cmd+Enter). Shift + that chord inserts a blank line above. Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.183] — 2026-10-10

Edit / Settings:

- **Invert Case / Proper Case** is remappable (`shortcut_invert_case`, default Cmd+Alt+U). Shift + that chord runs Proper Case. Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.182] — 2026-10-10

Edit / Settings:

- **Join Lines / Split Lines** is remappable (`shortcut_join_lines`, default Cmd+J). Shift + that chord runs Split Lines. Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.181] — 2026-10-10

Edit / Settings:

- **lowercase / UPPERCASE** is remappable (`shortcut_lowercase`, default Cmd+U). Shift + that chord runs UPPERCASE. Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.180] — 2026-10-10

Search / Settings:

- **Next / Previous Change History** is remappable (`shortcut_next_changed`, default F8). Shift + that chord jumps to the previous change-history mark. Menu, Preferences, Shortcut Mapper, and About show the effective binding.

## [0.3.179] — 2026-10-10

View / Compare:

- **Focus on Another View** now toggles keyboard focus between dual-view panes (matches the menu label) instead of swapping tabs. Remappable (`shortcut_focus_other_view`, default F6). Shift + that chord swaps tabs (header Switch). Other-view header shows ` · focused` when that pane has focus, plus a Focus button. Status names the focused side (`primary` / `other`, or `L` / `R` while Compare is on).

## [0.3.178] — 2026-10-10

View / Settings:

- **Go to document start / end** is remappable (`shortcut_goto_document_start`, default Cmd+Up). Opposite Down jumps to end; letter remaps use Shift for end. Menu, Preferences, Shortcut Mapper, and About show the effective binding. `Cmd+Home` / `Cmd+End` stay hard-wired aliases.

## [0.3.177] — 2026-10-10

View:

- **Document Map** viewport band is sized to the editor page (visible rows / file lines) with a soft green wash, not a fixed ~8% mark. Title appends `· view N` when the page is shorter than the file; the band fills the map when the whole file fits.

## [0.3.176] — 2026-10-10

Search / Settings:

- **Find in Files** is remappable (`shortcut_find_in_files`, default Cmd+Alt+F). Menu, Find bar button, Preferences, Shortcut Mapper, and the chord run the workspace scan (empty Find text still opens the Find bar).

## [0.3.175] — 2026-10-10

View / Settings:

- **Document Map** toggle is remappable (`shortcut_document_map`, default Cmd+Shift+D). Menu, Preferences, Shortcut Mapper, and the chord all open/close the panel (no longer open-only).

## [0.3.174] — 2026-10-10

View:

- **Document Map** paints a soft blue wash over the current selection (title `· sel`), a white caret tick at the focused caret, and orange center ticks for multi-carets when column / multi-select is active (title `· N carets`). Click near a caret tick parks on that line (status `Document Map → caret line N` / `… · N carets`); while Compare is on, the click also syncs the partner pane like a hunk/Equal park.

## [0.3.173] — 2026-10-10

View / Fold:

- **Document Map** paints slate fold-header ticks (brighter when that region is folded). Title appends fold count (`· N folds` / `· N folds (M folded)`). Click near a tick parks the caret and toggles that fold (status `Document Map → folded|unfolded K lines at N`); while Compare is on, the click also syncs the partner pane like a hunk/Equal park.

## [0.3.172] — 2026-10-10

Search / View:

- **Document Map** paints magenta ticks on Find match lines when the Find query has hits. Title appends match count. Click near a tick selects the first match on that line (status `Document Map → find line N · K matches`); while Compare is on, the click also syncs the partner pane like a hunk/Equal park.

## [0.3.171] — 2026-10-09

View:

- **Document Map** paints change-history ticks on the right (amber unsaved, green saved, same hues as the gutter bars). Title appends change count. Click near a tick parks the caret on that line (status `Document Map → change history line N (unsaved|saved)`); while Compare is on, the click also syncs the partner pane like a hunk/Equal park. Bookmark ticks stay on the left.

## [0.3.170] — 2026-10-09

View:

- **Document Map** paints cyan ticks at bookmark lines (same hue as the gutter mark). Title appends bookmark count. Click near a tick parks the caret on that bookmark (status `Document Map → bookmark line N`); while Compare is on, the click also syncs the partner pane like a hunk/Equal park.

## [0.3.169] — 2026-10-09

Compare / View:

- **Document Map** while Compare + Hide Unchanged Lines is on paints muted bands over collapsed Equal runs (`···N` gaps). Title shows gap count. Click a band to park on that gap (same status shape as Alt+F7 hidden-equal nav); other Compare clicks still park hunk / Equal partner.

## [0.3.168] — 2026-10-09

Compare / View:

- **Document Map** while Compare is on paints amber tick marks at each change-hunk start so clusters read at a glance. Drag (and non-Compare click) also syncs the partner pane when sync H/V scroll is on (Compare starts with sync V).

## [0.3.167] — 2026-10-09

Compare / View:

- **Document Map** click while Compare is on parks the caret on that line and syncs the other pane (hunk select or Equal partner park, same status as a gutter click). Drag still only scrolls.

## [0.3.166] — 2026-10-09

Compare / View:

- **Document Map** shows 2-way Compare colours while Compare is on: delete lines red, insert lines green, equal lines stay density gray. Map follows the focused dual-view pane (scroll mark + jump). Title appends `· compare` when tags apply.

## [0.3.165] — 2026-10-09

Plugins / local AI:

- **Ask Ollama** and **Ollama Status** under Plugins: talk to a user-run Ollama on loopback only (`127.0.0.1` / `localhost` / `::1`). Selection (or whole buffer) goes to `/api/generate`; reply opens in a tab. Status lists `/api/tags` models.
- Preferences / `settings.json`: `ollama_host` (default `http://127.0.0.1:11434`), `ollama_model` (default `llama3.2`). Remote hosts rejected. See `docs/ollama-helper.md` (issue #18).

## [0.3.164] — 2026-10-09

UI / menus:

- Tall menus (especially **View**) stay on screen: popup bodies use a vertical scroll area capped to the window height so egui does not flip them upward over the menu bar.
- Remappable shortcuts show on the right of matching menu items (weak shortcut text).

## [0.3.163] — 2026-10-09

Editor:

- Vertical **scrollbar** in the edit window is clickable and draggable: click the track to jump, drag the thumb to scroll (primary and dual-view secondary panes). Wider hit strip so the thin thumb is easier to grab.

## [0.3.162] — 2026-10-09

Settings / shortcuts:

- **Bookmark / Clear Compare Difference Bookmarks** is remappable: Preferences → Bookmark compare differences shortcut, or `shortcut_bookmark_compare_diffs` in `npp-rs/settings.json` (default `Alt+M`). Shift flips to Clear (`Alt+Shift+M`). Shortcut Mapper and About show the effective binding.

## [0.3.161] — 2026-10-09

Help / About:

- **About** is clean again: version, links, short why/made-by copy. Keyboard shortcuts moved to a separate **Keyboard shortcuts** window (About → Keyboard shortcuts…), with a vertical scroll area when the list is tall. Settings → Shortcut Mapper still opens the text dump.

## [0.3.160] — 2026-10-09

Settings / shortcuts:

- **Expand / Collapse Unchanged at Caret** (Compare) is remappable: Preferences → Expand unchanged at caret shortcut, or `shortcut_expand_unchanged_at_caret` in `npp-rs/settings.json` (default `Alt+X`). Shift flips to Collapse at Caret (`Alt+Shift+X`). Shortcut Mapper and About show the effective binding.

## [0.3.159] — 2026-10-09

Settings / shortcuts:

- **Expand / Collapse All Unchanged Lines** (Compare) is remappable: Preferences → Expand all unchanged shortcut, or `shortcut_expand_all_unchanged` in `npp-rs/settings.json` (default `Alt+E`). Shift flips to Collapse All (`Alt+Shift+E`). Shortcut Mapper and About show the effective binding.

## [0.3.158] — 2026-10-09

Settings / shortcuts:

- **Ignore blank lines** (Compare) is remappable: Preferences → Ignore blank lines shortcut, or `shortcut_compare_ignore_blank` in `npp-rs/settings.json` (default `Alt+B`). Shortcut Mapper and About show the effective binding.

## [0.3.157] — 2026-10-09

Settings / shortcuts:

- **Ignore whitespace / Ignore case** (Compare) is remappable: Preferences → Ignore whitespace shortcut, or `shortcut_compare_ignore_ws` in `npp-rs/settings.json` (default `Alt+W`). Shift flips to Ignore Case (`Alt+Shift+W`). Ignore Blank Lines stayed menu-only until v0.3.158. Shortcut Mapper and About show the effective binding.

## [0.3.156] — 2026-10-09

Settings / shortcuts:

- **Tab indent** is remappable: Preferences → Tab indent shortcut, or `shortcut_tab_indent` in `npp-rs/settings.json` (default `Tab`). Inserts spaces at the caret (and multi-carets); `Cmd+]` (`shortcut_indent`) stays as line-indent. Shortcut Mapper and About show the effective binding.

## [0.3.155] — 2026-10-09

Settings / shortcuts:

- **Copy / Open Compare Hunk** is remappable: Preferences → Copy Compare Hunk shortcut, or `shortcut_copy_compare_hunk` in `npp-rs/settings.json` (default `Alt+K`). Shift flips to Open Compare Hunk (`Alt+Shift+K`). Shortcut Mapper and About show the effective binding.

## [0.3.154] — 2026-10-09

Settings / shortcuts:

- **Copy / Open Compare Summary** is remappable: Preferences → Copy Compare Summary shortcut, or `shortcut_copy_compare_summary` in `npp-rs/settings.json` (default `Alt+Y`). Shift flips to Open Compare Summary (`Alt+Shift+Y`). Shortcut Mapper and About show the effective binding.

## [0.3.153] — 2026-10-09

Settings / shortcuts:

- **Copy / Open Compare Diff** is remappable: Preferences → Copy Compare Diff shortcut, or `shortcut_copy_compare_diff` in `npp-rs/settings.json` (default `Alt+C`). Shift flips to Open Compare Diff (`Alt+Shift+C`). Shortcut Mapper and About show the effective binding.

## [0.3.152] — 2026-10-09

Settings / shortcuts:

- **Toggle comment** is remappable: Preferences → Toggle comment shortcut, or `shortcut_toggle_comment` in `npp-rs/settings.json` (default `Cmd+/`). Shift flips to Block Comment (`Cmd+Shift+/`). Shortcut Mapper and About show the effective binding. Uses `/` so macOS Cmd+Q Quit stays free (N++ Ctrl+Q remaps via Preferences).

## [0.3.151] — 2026-10-09

Settings / shortcuts:

- **Move line** is remappable: Preferences → Move line shortcut, or `shortcut_move_line` in `npp-rs/settings.json` (default `Cmd+Shift+Up`). Opposite Down moves the line down; letter remaps use Shift for down. Shortcut Mapper and About show the effective binding. `Cmd+Up` / `Cmd+Down` still jump to document start / end.

## [0.3.150] — 2026-10-09

Settings / shortcuts:

- **Matching brace** is remappable: Preferences → Matching brace shortcut, or `shortcut_matching_brace` in `npp-rs/settings.json` (default `Cmd+B`). Shift flips to Select matching braces (`Cmd+Shift+B`). Shortcut Mapper and About show the effective binding.

## [0.3.149] — 2026-10-09

Settings / shortcuts:

- **Fold / Unfold current** is remappable: Preferences → Fold current shortcut, or `shortcut_fold_current` in `npp-rs/settings.json` (default `Alt+F`). Shift flips to Unfold Current (`Alt+Shift+F`). Shortcut Mapper and About show the effective binding.

## [0.3.148] — 2026-10-09

Settings / shortcuts:

- **Fold / Unfold all** is remappable: Preferences → Fold all shortcut, or `shortcut_fold_all` in `npp-rs/settings.json` (default `Alt+0`). Shift flips to Unfold All (`Alt+Shift+0`). Shortcut Mapper and About show the effective binding.

## [0.3.147] — 2026-10-09

Settings / shortcuts:

- **Shift+Tab outdent** is remappable: Preferences → Shift+Tab outdent shortcut, or `shortcut_shift_tab_outdent` in `npp-rs/settings.json` (default `Shift+Tab`). Outdents the current line(s); `Cmd+[` (`shortcut_outdent`) stays as the other outdent binding. Shortcut Mapper and About show the effective binding.

## [0.3.146] — 2026-10-09

Settings / shortcuts:

- **Print** is remappable: Preferences → Print shortcut, or `shortcut_print` in `npp-rs/settings.json` (default `Cmd+P`). Shortcut Mapper and About show the effective binding.

## [0.3.145] — 2026-10-09

Settings / shortcuts:

- **Save All** is remappable: Preferences → Save All shortcut, or `shortcut_save_all` in `npp-rs/settings.json` (default `Cmd+Alt+S`). Shortcut Mapper and About show the effective binding.

## [0.3.144] — 2026-10-09

Settings / shortcuts:

- **Next / previous tab** is remappable: Preferences → Next tab shortcut, or `shortcut_next_tab` in `npp-rs/settings.json` (default `Cmd+Tab`). Shift flips to previous tab. Shortcut Mapper and About show the effective binding.

## [0.3.143] — 2026-10-09

Settings / shortcuts:

- **Compare to Saved** is remappable: Preferences → Compare to Saved shortcut, or `shortcut_compare_to_saved` in `npp-rs/settings.json` (default `Alt+Shift+S`). Shortcut Mapper and About show the effective binding.

## [0.3.142] — 2026-10-09

Settings / shortcuts:

- **Word jump** is remappable: Preferences → Word jump shortcut, or `shortcut_word_jump` in `npp-rs/settings.json` (default `Alt+Left`). Opposite Left/Right jumps forward; Shift extends the selection. Shortcut Mapper and About show the effective binding.

## [0.3.141] — 2026-10-08

Settings / shortcuts:

- **Swap Compare Sides** is remappable: Preferences → Swap Compare sides shortcut, or `shortcut_swap_compare` in `npp-rs/settings.json` (default `Alt+S`). Shortcut Mapper and About show the effective binding.

## [0.3.140] — 2026-10-08

Settings / shortcuts:

- **Close Find/Replace** is remappable: Preferences → Close Find/Replace shortcut, or `shortcut_close_find` in `npp-rs/settings.json` (default `Escape`). Shortcut Mapper and About show the effective binding.

## [0.3.139] — 2026-10-08

macOS install / Gatekeeper:

- Release pack keeps the execute bit on Unix binaries; macOS artifact is ad-hoc codesigned in CI
- One-liner install: `scripts/install-macos.sh` (download, clear quarantine, install to Applications)
- Docs: `docs/macos-install.md` — Gatekeeper “damaged” is quarantine, not a corrupt file

## [0.3.138] — 2026-10-08

Settings / shortcuts:

- **Compare start / clear** is remappable: Preferences → Compare shortcut, or `shortcut_compare` in `npp-rs/settings.json` (default `Alt+D`). Shift + that chord is Clear Compare. Shortcut Mapper and About show the effective binding.

## [0.3.137] — 2026-10-08

Settings / shortcuts:

- **Hide unchanged lines** is remappable: Preferences → Hide unchanged lines shortcut, or `shortcut_hide_equal` in `npp-rs/settings.json` (default `Alt+H`). Shortcut Mapper and About show the effective binding.

## [0.3.136] — 2026-10-08

Settings / shortcuts:

- **First hidden equal** is remappable: Preferences → First hidden equal shortcut, or `shortcut_first_hidden_equal` in `npp-rs/settings.json` (default `Cmd+Alt+F7`). Shift + that chord is Last hidden equal. Shortcut Mapper and About show the effective binding.

## [0.3.135] — 2026-10-08

Settings / shortcuts:

- **First difference** is remappable: Preferences → First difference shortcut, or `shortcut_first_diff` in `npp-rs/settings.json` (default `Cmd+F7`). Shift + that chord is Last difference. Shortcut Mapper and About show the effective binding.

## [0.3.134] — 2026-10-08

Settings / shortcuts:

- **Reload from disk** is remappable: Preferences → Reload shortcut, or `shortcut_reload` in `npp-rs/settings.json` (default `Cmd+R`). Shortcut Mapper and About show the effective binding.

## [0.3.133] — 2026-10-08

Settings / shortcuts:

- **Hide-equal context** is remappable: Preferences → Hide-equal context shortcut, or `shortcut_hide_equal_context` in `npp-rs/settings.json` (default `Alt+]`). Opposite `[` / `]` decreases. Shortcut Mapper and About show the effective bindings.

## [0.3.132] — 2026-10-08

Settings / shortcuts:

- **Apply compare hunk** is remappable: Preferences → Apply compare hunk shortcut, or `shortcut_apply_compare_hunk` in `npp-rs/settings.json` (default `Cmd+Alt+Left`). Opposite Left/Right applies the hunk to the other pane; Shift applies all hunks in that direction. Chord parser accepts `Left` / `Right` / `ArrowLeft` / `ArrowRight`. Shortcut Mapper and About show the effective bindings.

## [0.3.131] — 2026-10-08

Settings / shortcuts:

- **Next hidden equal** is remappable: Preferences → Next hidden equal shortcut, or `shortcut_next_hidden_equal` in `npp-rs/settings.json` (default `Alt+F7`). Shift + that chord is Previous hidden equal. Cmd/Ctrl+Alt+F7 first/last stay hard-wired. Shortcut Mapper and About show the effective binding.

## [0.3.130] — 2026-10-08

Settings / shortcuts:

- **Replace (alternate)** is remappable: Preferences → Replace (alternate) shortcut, or `shortcut_replace_alt` in `npp-rs/settings.json` (default `Cmd+Shift+F`). Shortcut Mapper and About show the effective binding.

## [0.3.129] — 2026-10-08

Settings / shortcuts:

- **Find next (global)** is remappable: Preferences → Find next (global) shortcut, or `shortcut_find_next_global` in `npp-rs/settings.json` (default `Cmd+G`). Shift + that chord is Find previous. Shortcut Mapper and About show the effective binding.

## [0.3.128] — 2026-10-08

Settings / shortcuts:

- **Toggle log tail** is remappable: Preferences → Toggle log tail shortcut, or `shortcut_toggle_log_tail` in `npp-rs/settings.json` (default `Cmd+Shift+T`). Shortcut Mapper and About show the effective binding.

## [0.3.127] — 2026-10-08

Settings / shortcuts:

- **Zoom restore** is remappable: Preferences → Zoom restore shortcut, or `shortcut_zoom_restore` in `npp-rs/settings.json` (default `Cmd+0`). Mouse wheel zoom stays hard-wired. Shortcut Mapper and About show the effective binding.

## [0.3.126] — 2026-10-08

Settings / shortcuts:

- **Zoom out** is remappable: Preferences → Zoom out shortcut, or `shortcut_zoom_out` in `npp-rs/settings.json` (default `Cmd+-`). Restore (`Cmd+0`) stays hard-wired. Shortcut Mapper and About show the effective binding.

## [0.3.125] — 2026-10-08

Settings / shortcuts:

- **Zoom in** is remappable: Preferences → Zoom in shortcut, or `shortcut_zoom_in` in `npp-rs/settings.json` (default `Cmd+=`). Zoom out (`Cmd+-`) and restore (`Cmd+0`) stay hard-wired. Shortcut Mapper and About show the effective binding.

## [0.3.124] — 2026-10-08

Compare:

- Files longer than **3000 lines** still Compare: LCS uses the first 3000 lines per side (tail stays uncoloured). Live status appends `· first 3000 lines` instead of refusing the pair.

## [0.3.123] — 2026-10-08

Settings / shortcuts:

- **Redo** is remappable: Preferences → Redo shortcut, or `shortcut_redo` in `npp-rs/settings.json` (default `Cmd+Y`). Shift + Undo chord remains Redo. Shortcut Mapper and About show the effective binding.

## [0.3.122] — 2026-10-08

Settings / shortcuts:

- **Undo** is remappable: Preferences → Undo shortcut, or `shortcut_undo` in `npp-rs/settings.json` (default `Cmd+Z`). Shift + that chord is Redo. Cmd/Ctrl+Y stays hard-wired as an alternate. Shortcut Mapper and About show the effective binding.

## [0.3.121] — 2026-10-08

Settings / shortcuts:

- **Select all** is remappable: Preferences → Select all shortcut, or `shortcut_select_all` in `npp-rs/settings.json` (default `Cmd+A`). Shortcut Mapper and About show the effective binding.

## [0.3.120] — 2026-10-08

Settings / shortcuts:

- **Open** is remappable: Preferences → Open shortcut, or `shortcut_open` in `npp-rs/settings.json` (default `Cmd+O`). Shortcut Mapper and About show the effective binding.

## [0.3.119] — 2026-10-08

Settings / shortcuts:

- **New** is remappable: Preferences → New shortcut, or `shortcut_new` in `npp-rs/settings.json` (default `Cmd+N`). Shortcut Mapper and About show the effective binding.

## [0.3.118] — 2026-10-08

Settings / shortcuts:

- **Save As** is remappable: Preferences → Save As shortcut, or `shortcut_save_as` in `npp-rs/settings.json` (default `Cmd+Shift+S`). Shortcut Mapper and About show the effective binding.

## [0.3.117] — 2026-10-08

Settings / shortcuts:

- **Replace** is remappable: Preferences → Replace shortcut, or `shortcut_replace` in `npp-rs/settings.json` (default `Cmd+H`). Cmd+Shift+F stays hard-wired as an alternate. Shortcut Mapper and About show the effective binding.

## [0.3.116] — 2026-10-08

Settings / shortcuts:

- **Find** is remappable: Preferences → Find shortcut, or `shortcut_find` in `npp-rs/settings.json` (default `Cmd+F`). Replace stays hard-wired `Cmd+H` / `Cmd+Shift+F`. Shortcut Mapper and About show the effective binding.

## [0.3.115] — 2026-10-08

Settings / shortcuts:

- **Save** is remappable: Preferences → Save shortcut, or `shortcut_save` in `npp-rs/settings.json` (default `Cmd+S`). Save As stays hard-wired `Cmd+Shift+S`. Shortcut Mapper and About show the effective binding.

## [0.3.114] — 2026-10-07

Settings / shortcuts:

- **Close tab** is remappable: Preferences → Close tab shortcut, or `shortcut_close_tab` in `npp-rs/settings.json` (default `Cmd+W`). Shortcut Mapper and About show the effective binding.

## [0.3.113] — 2026-10-07

Settings / shortcuts:

- **Format document** is remappable: Preferences → Format document shortcut, or `shortcut_format_document` in `npp-rs/settings.json` (default `Cmd+Shift+I`). Shortcut Mapper and About show the effective binding.

## [0.3.112] — 2026-10-07

Settings / shortcuts:

- **Toggle bookmark** is remappable: Preferences → Toggle bookmark shortcut, or `shortcut_toggle_bookmark` in `npp-rs/settings.json` (default `Cmd+F2`). Shortcut Mapper and About show the effective binding.

## [0.3.111] — 2026-10-07

Settings / shortcuts:

- **Outdent** is remappable: Preferences → Outdent shortcut, or `shortcut_outdent` in `npp-rs/settings.json` (default `Cmd+[`). Shortcut Mapper and About show the effective binding.

## [0.3.110] — 2026-10-07

Settings / shortcuts:

- **Indent** is remappable: Preferences → Indent shortcut, or `shortcut_indent` in `npp-rs/settings.json` (default `Cmd+]`). Outdent stays hard-wired `Cmd+[`. Shortcut Mapper and About show the effective binding.

## [0.3.109] — 2026-10-07

Settings / shortcuts:

- **Delete line** is remappable: Preferences → Delete line shortcut, or `shortcut_delete_line` in `npp-rs/settings.json` (default `Cmd+Shift+L`). Shortcut Mapper and About show the effective binding.

## [0.3.108] — 2026-10-07

Settings / shortcuts:

- **Duplicate line** is remappable: Preferences → Duplicate line shortcut, or `shortcut_duplicate_line` in `npp-rs/settings.json` (default `Cmd+D`). Shortcut Mapper and About show the effective binding.

## [0.3.107] — 2026-10-07

Settings / shortcuts:

- **Go to line** is remappable: Preferences → Go to line shortcut, or `shortcut_goto_line` in `npp-rs/settings.json` (default `Cmd+L`). Cmd/Ctrl+Shift+L delete line stays hard-wired. Shortcut Mapper and About show the effective binding.

## [0.3.106] — 2026-10-07

Settings / shortcuts:

- **Next difference** is remappable: Preferences → Next difference shortcut, or `shortcut_next_diff` in `npp-rs/settings.json` (default `F7`). Shift + that chord is Previous difference. Cmd/Ctrl+F7 first/last and Alt+F7 hidden-equal stay hard-wired. Shortcut Mapper and About show the effective binding.

## [0.3.105] — 2026-10-07

Settings / shortcuts:

- **Next bookmark** is remappable: Preferences → Next bookmark shortcut, or `shortcut_next_bookmark` in `npp-rs/settings.json` (default `F2`). Shift + that chord is Previous bookmark. Cmd/Ctrl+F2 toggle stays hard-wired. Shortcut Mapper and About show the effective binding.

## [0.3.104] — 2026-10-07

Settings / shortcuts:

- **Find next** is remappable: Preferences → Find next shortcut, or `shortcut_find_next` in `npp-rs/settings.json` (default `F3`). Shift + that chord is Find previous. Cmd/Ctrl+G stays hard-wired. Shortcut Mapper and About show the effective binding.

## [0.3.103] — 2026-10-07

Compare:

- Equal-line partner park (click / keyboard) status appends the same ignore / hide-equal bits as the live pair line, e.g. `Compare equal → L12 | R12 (−1 +2, 2 hunks: 1 delete, 1 insert) · 50% equal · ignore ws+case · hide equal ±3 · 5 hidden` (identical pairs keep `(identical)` plus those bits when active).

## [0.3.102] — 2026-10-07

Compare:

- Equal-line partner park (click / keyboard) status appends Equal-line match percent, e.g. `Compare equal → L12 | R12 (−1 +2, 2 hunks: 1 delete, 1 insert) · 50% equal` (identical pairs stay `(identical)`).

## [0.3.101] — 2026-10-07

Compare:

- Equal-line partner park (click / keyboard) status includes pair −/+/kind tallies, e.g. `Compare equal → L12 | R12 (−1 +2, 2 hunks: 1 delete, 1 insert)` (or `(identical)`).

## [0.3.100] — 2026-10-07

Compare:

- Equal-line partner park (click / keyboard / edit re-diff) auto-reveals a Hide-Unchanged collapsed Equal line on either pane so the parked caret stays visible. Edit re-diff on Equal appends `· equal L|R` to pair status (hunk landings still use `· at L|R`).

## [0.3.99] — 2026-10-07

Compare:

- After the ~200 ms edit re-diff, the other pane parks on the focused caret’s change hunk (or Equal partner); pair status appends `· at L|R (i/n kind −/+)`. Focused caret and selection stay put so typing is not yanked.

## [0.3.98] — 2026-10-07

Compare:

- Changing **Hide-equal context** (Alt+]/ / Alt+[ or Preferences slider) while Compare + Hide Unchanged Lines is on parks **and selects** both panes on the first change hunk; status is `Hide-equal context ±N — Compare … · at L|R (1/n kind −/+)`.

## [0.3.97] — 2026-10-07

Compare:

- **Hide Unchanged Lines** while Compare is on parks **and selects** both panes on the first change hunk; status appends `· at L|R (1/n kind −/+)` like ignore re-diff / Compare start / swap sides.

## [0.3.96] — 2026-10-07

Compare:

- **Next / Previous / First / Last Hidden Equal** hard-wired shortcuts: Alt+F7 / Alt+Shift+F7 walk ···N gaps; ⌘/Ctrl+Alt+F7 / ⌘/Ctrl+Alt+Shift+F7 jump to first/last. Plain F7 family still walks change hunks. About, Shortcut Mapper, and menu hover list the bindings.

## [0.3.95] — 2026-10-07

Compare:

- **Apply All Hunks From / To Other View** hard-wired shortcuts: ⌘/Ctrl+Alt+Shift+← pulls every remaining hunk from the other pane; ⌘/Ctrl+Alt+Shift+→ pushes every remaining hunk to the other pane. Single-hunk ⌘/Ctrl+Alt+←/→ unchanged. About, Shortcut Mapper, and menu hover list the bindings.

## [0.3.94] — 2026-10-07

Compare:

- **Apply Hunk From / To Other View** hard-wired shortcuts: ⌘/Ctrl+Alt+← pulls the other pane into the focused hunk; ⌘/Ctrl+Alt+→ pushes the focused hunk to the other pane. Alt alone still word-jumps. About, Shortcut Mapper, and menu hover list the bindings.

## [0.3.93] — 2026-10-06

Compare:

- **Next / Previous / First / Last Hidden Equal** status includes −/+ and hunk-kind tallies (`Compare Next hidden equal → ···5 (1/3) (−1 +2, 2 hunks: 1 delete, 1 insert) · wrapped`).

## [0.3.92] — 2026-10-06

Compare:

- **Expand Unchanged at Caret** / click ···N / **Collapse Unchanged at Caret** status includes −/+ and hunk-kind tallies (`Compare expanded ···5 both panes (−1 +2, 2 hunks: 1 delete, 1 insert, 8 still hidden)` / `Compare collapsed ···5 both panes (−1 +2, 2 hunks: 1 delete, 1 insert, 8 hidden)`).

## [0.3.91] — 2026-10-06

Compare:

- **Expand All Unchanged Lines** / **Collapse All Unchanged Lines** status includes −/+ and hunk-kind tallies (`Compare expanded all unchanged lines (−1 +2, 2 hunks: 1 delete, 1 insert, 12 shown)` / `Compare collapsed expanded equal lines (−1 +2, 2 hunks: 1 delete, 1 insert, 12 re-hidden, 8 hidden)`).

## [0.3.90] — 2026-10-06

Compare:

- **Bookmark Compare Differences** / **Clear Compare Difference Bookmarks** status includes −/+ and hunk-kind tallies (`Compare bookmarked differences (−1 +2, L2|R2, 2 hunks: 1 delete, 1 insert, +3 new)` / `Compare cleared difference bookmarks (−1 +2, L2|R2, 2 hunks: 1 delete, 1 insert, −3 removed)`).

## [0.3.89] — 2026-10-06

Compare:

- **Clear Compare** status keeps the last pair overview with −/+ and hunk-kind tallies (`Compare cleared (−1 +2, 2 hunks: 1 delete, 1 insert) “a” | “b”`, or `(identical)`; saved-snapshot clear keeps the same counts after the closed-snapshot prefix).

## [0.3.88] — 2026-10-06

Compare:

- **Swap Compare Sides** re-diffs and parks **and selects** both panes on the first change hunk; status appends `· at L|R (1/n kind −/+)` like Compare start / ignore re-diff.

## [0.3.87] — 2026-10-06

Compare:

- While Compare is on, toggling Ignore Whitespace / Case / Blank Lines re-diffs and parks **and selects** both panes on the first remaining change hunk; status appends `· at L|R (1/n kind −/+)` like Compare start.

## [0.3.86] — 2026-10-06

Compare:

- Starting Compare (and Compare to Saved) status appends the parked first hunk (`· at L12 | R15 (1/5 replace −1 +1)`), matching nav/click ordinal bits.

## [0.3.85] — 2026-10-06

Compare:

- **Copy Compare Diff** / **Open Compare Diff** status includes hunk-kind tallies (`Copied unified diff (−1 +2, 2 hunks: 1 delete, 1 insert) …`), matching Copy/Open Compare Summary.

## [0.3.84] — 2026-10-06

Compare:

- **Copy Compare Summary** / **Open Compare Summary** status includes hunk-kind tallies (`Copied compare summary (−1 +2, 2 hunks: 1 delete, 1 insert) …`), matching the live Compare status / summary header.

## [0.3.83] — 2026-10-06

Compare:

- **Copy Compare Hunk** / **Open Compare Hunk** status includes the hunk kind and −/+ counts (`Copied hunk (2/5 replace −1 +1) unified diff …`), matching nav/apply ordinal bits.

## [0.3.82] — 2026-10-06

Compare:

- **Apply All Hunks From/To Other View** status includes kind tallies and total −/+ for the hunks actually applied (`Applied 3 hunks (1 delete, 1 insert, 1 replace −2 +2) from other view · identical`).

## [0.3.81] — 2026-10-06

Compare:

- **Apply Hunk From/To Other View** status includes the applied hunk kind and −/+ counts, and the next remaining hunk uses the same ordinal bit (`Applied hunk (1/5 replace −2 +1) from other view → L12 | R15 (2/4 delete −1 +0)`).

## [0.3.80] — 2026-10-06

Compare:

- Next/Previous/First/Last Difference and click/keyboard hunk sync status include the current hunk −/+ line counts (`(2/5 replace −1 +1)`), so hunk size is visible without opening Compare Summary.

## [0.3.79] — 2026-10-06

Compare:

- Next/Previous/First/Last Difference and click/keyboard hunk sync status include the current hunk kind (`(2/5 replace)`), so delete/insert/replace is visible without opening Compare Summary.

## [0.3.78] — 2026-10-06

Compare:

- Live Compare status line tallies hunk kinds after the hunk count (`· K hunks: N delete, M insert, P replace`) so delete/insert/replace mix is visible without opening Compare Summary.

## [0.3.77] — 2026-10-06

Compare:

- Live Compare status line appends Equal-line match percent (`· N% equal`) for non-identical pairs so similarity is visible without opening Compare Summary.

## [0.3.76] — 2026-10-06

Compare:

- **Open / Copy Compare Summary** header includes an Equal-line match percent (`· N% equal`) so similarity is visible without scanning every hunk.

## [0.3.75] — 2026-10-06

Compare:

- **Open / Copy Compare Summary** shows up to 3 preview lines per side under each hunk (`-` / `+`, max 72 chars), then `… (+N more)` when the side is longer, so multi-line hunks stay scannable.

## [0.3.74] — 2026-10-05

Compare:

- **Open / Copy Compare Summary** header appends active ignore flags (`· ignore ws+case+blank`) so a copied or opened summary records which Ignore toggles shaped the hunk list.

## [0.3.73] — 2026-10-05

Compare:

- **Open / Copy Compare Summary** header tallies hunk kinds (`N delete, M insert, K replace`) next to the hunk count.

## [0.3.72] — 2026-10-05

Compare:

- **Open / Copy Compare Summary** includes a truncated first-line preview under each hunk (`-` / `+`, max 72 chars) so the index is scannable without opening every hunk.

## [0.3.71] — 2026-10-05

Compare:

- **View → Copy Compare Summary** copies the hunk index text to the clipboard (same body as Open Compare Summary; Compare stays on).

## [0.3.70] — 2026-10-05

Compare:

- **View → Open Compare Summary** opens a `compare-summary.txt` tab listing every change hunk with L|R line ranges and −/+ counts (clears Compare so the tab stays visible).

## [0.3.69] — 2026-10-05

Compare / Preferences:

- **Hide-equal context** (0–10, default 3) sets how many Equal lines stay visible on each side of a change when Hide Unchanged Lines is on. Status shows `· hide equal ±N`. Key: `compare_hide_equal_context`.

## [0.3.68] — 2026-10-05

Compare:

- **View → First Hidden Equal** / **Last Hidden Equal** jump to the first/last ···N collapsed Equal gap while Hide Unchanged Lines is on (parks both panes; status shows ordinal).

## [0.3.67] — 2026-10-05

Compare:

- **View → Next Hidden Equal** / **Previous Hidden Equal** jump between ···N collapsed Equal gaps while Hide Unchanged Lines is on (wraps with `· wrapped` status; parks both panes on the aligned Equal partner).

## [0.3.66] — 2026-10-05

Folding / session:

- Opt-in session restore now keeps **folded regions** (writes `@folds` header lines under each path in `npp-rs/session.txt`; applies on restore when headers still match).

## [0.3.65] — 2026-10-05

Compare:

- **View → Collapse Unchanged at Caret** re-collapses the expanded Equal run nearest the caret on both panes (···N cue returns for that run; hide-equal stays on).

## [0.3.64] — 2026-10-05

Compare:

- **View → Expand Unchanged at Caret** expands the collapsed Equal run nearest the caret on both panes (same as clicking that `···N` cue; hide-equal stays on).

## [0.3.63] — 2026-10-05

Compare:

- Click or move the caret onto an Equal (unchanged) line parks the other pane on the LCS-aligned partner line (status: `Compare equal → Lx | Ry`). Change-line hunk select is unchanged.

## [0.3.62] — 2026-10-05

Compare:

- **View → Clear Compare Difference Bookmarks** removes bookmarks at each change-hunk start on both panes (other bookmarks stay).

## [0.3.61] — 2026-10-05

Compare:

- **View → Bookmark Compare Differences** bookmarks the start of every change hunk on both panes so F2 / Shift+F2 can walk differences after you leave a hunk.

## [0.3.60] — 2026-10-05

Compare:

- **View → Collapse All Unchanged Lines** re-collapses every click/menu-expanded Equal run on both panes while Hide Unchanged Lines stays on (`···N` cues return without flipping the preference).

## [0.3.59] — 2026-10-05

Compare:

- **View → Expand All Unchanged Lines** reveals every collapsed Equal run on both panes while Hide Unchanged Lines stays on (clears remaining `···N` cues without flipping the preference).

## [0.3.58] — 2026-10-05

Compare:

- **Hide Unchanged Lines**: clicking a `···N` cue expands the collapsed Equal run on **both panes** (LCS-aligned partner lines). Status notes `both panes` when the other side opened too.

## [0.3.57] — 2026-10-05

Compare:

- **Hide Unchanged Lines**: click a gutter `···N` cue to expand that collapsed Equal run on the clicked pane. Status reports how many lines opened and how many remain hidden. Clear Compare / toggle hide-equal off / swap sides resets expansions.

## [0.3.56] — 2026-10-05

Compare:

- **Hide Unchanged Lines** paints gutter `···N` gap cues (and a hairline) between collapsed Equal runs, and status shows how many lines are hidden (`· N hidden` or `· Lx|Ry hidden`).

## [0.3.55] — 2026-10-05

Compare:

- **Hide Unchanged Lines** keeps **±3 Equal lines of context** around each change hunk so collapsed Compare panes stay readable. Status shows `· hide equal ±3`.

## [0.3.54] — 2026-10-05

Compare:

- **View → Hide Unchanged Lines** (Preferences persist) collapses Equal-tagged lines in both Compare panes so only change lines stay visible. Identical pairs stay fully shown. Status appends `· hide equal`.

## [0.3.53] — 2026-10-05

Compare:

- Replace-hunk intra-line wash is **word-aware**: LCS on word/separator tokens, with char refine on 1:1 token replaces so typos still highlight inside a word. Whole-word edits (e.g. `cat` → `dog`) wash the token, not scattered characters.

## [0.3.52] — 2026-10-05

Compare:

- **View → Clear Compare** (and other clear paths) closes the read-only `name (saved)` snapshot tab when it was part of the compare pair, so Compare to Saved does not leave an orphan tab.

## [0.3.51] — 2026-10-05

Compare:

- **View → Compare to Saved** diffs the active named tab against a read-only snapshot of its on-disk contents (reuses the `name (saved)` tab when present). Status uses the usual Compare −/+ / identical wording.

## [0.3.50] — 2026-10-05

Compare:

- **View → Apply All Hunks To Other View** pushes every remaining change hunk onto the other pane in one undo (last hunk first). Status shows how many hunks were applied, plus `· identical` when the pair matches.

## [0.3.49] — 2026-10-05

Compare:

- **View → Apply Hunk To Other View** pushes the change hunk at the focused caret onto the other pane (one undo). Parks and selects the next remaining hunk on both panes, same as Apply From Other. Status shows the next `L|R` ordinal, or `· identical` when the pair matches.

## [0.3.48] — 2026-10-05

Compare:

- **View → Open Compare Hunk** opens a `compare-hunk.diff` tab with the unified diff for the change hunk at the caret (same payload as Copy Compare Hunk). Compare mode turns off so the tab stays visible. Status shows hunk ordinal and −/+ counts.

## [0.3.47] — 2026-10-04

Compare:

- **View → Ignore Blank Lines** (Preferences `compare_ignore_blank`) skips blank / whitespace-only lines in the LCS so extra empty lines do not create hunks. Combines with ignore-whitespace / ignore-case; status shows `· ignore blank` (or `ws+case+blank`).

## [0.3.46] — 2026-10-04

Compare:

- **Apply Hunk From Other View** parks and selects the next remaining hunk on both panes after a successful apply. Status shows the next `L|R` ordinal, or `· identical` when the pair matches. **Apply All** also appends `· identical` when nothing is left.

## [0.3.45] — 2026-10-04

Search:

- Regex **Replace** / **Replace All** expand `$n`, `${n}`, `$0` / `$&` (whole match), `$$`, and `\1` (plus `\n` `\t` `\r`). Literal replace is unchanged when **Re** is off.

## [0.3.44] — 2026-10-04

Compare:

- **View → Open Compare Diff** opens a `compare.diff` tab with the same unified diff as Copy Compare Diff. Compare mode turns off so the tab stays visible. Status shows change counts or `(identical)`.

## [0.3.43] — 2026-10-04

Search:

- **Find in Files** honors the Find bar **Re** toggle (same linear-time regex as in-file Find). Invalid patterns report `Find in Files: invalid regex` and do not open a results tab. Literal search is unchanged when **Re** is off.

## [0.3.42] — 2026-10-04

Compare:

- **View → Apply All Hunks From Other View** copies every remaining change hunk onto the focused pane in one undo. Insert-only and delete-only hunks work. Status shows how many hunks were applied.

## [0.3.41] — 2026-10-04

Compare:

- Replace hunks highlight the **changed characters** on both panes (stronger wash on the intra-line LCS). Pure insert/delete lines stay whole-line colour. Lines over 256 characters keep the line wash only.

## [0.3.40] — 2026-10-04

Compare:

- **View → Apply Hunk From Other View** replaces the focused pane's change hunk with the other pane (one undo). Insert-only and delete-only hunks work. Status shows the hunk ordinal.

## [0.3.39] — 2026-10-04

Compare:

- **View → Copy Compare Hunk** copies the change hunk at the focused caret (or the next hunk) as a unified diff, with 3 equal context lines. Status shows hunk ordinal and −/+ counts.

## [0.3.38] — 2026-10-04

Compare:

- **View → Copy Compare Diff** copies a unified diff of the compared pair to the clipboard (3 lines of context). Status shows change counts or `(identical)`.

## [0.3.37] — 2026-10-04

Preferences:

- Unknown keys in `npp-rs/settings.json` survive the next save (hand-edited or future fields are not dropped).

## [0.3.36] — 2026-10-04

Compare:

- Next/Prev/First/Last Difference, click/keyboard hunk sync, and Start Compare select the change hunk on **both** panes (Copy/Delete still use the focused pane).

## [0.3.35] — 2026-10-04

Preferences / View:

- Show white space and TAB, Show EOL, Show NPC, and Indent guide persist in `npp-rs/settings.json` (View menu + Preferences) and restore on launch.

## [0.3.34] — 2026-10-04

Preferences:

- Recent file list load now honors Preferences **Recent file count** (`recent_max`, 5–40) instead of hard-capping at 15 on restart.

## [0.3.33] — 2026-10-04

Compare:

- Next/Prev/First/Last Difference selects the whole change hunk on the focused pane (Copy/Delete apply to the change).

## [0.3.32] — 2026-10-04

Compare:

- Next/Prev Difference status appends `· wrapped` when the jump circles past the end or beginning of the file.

## [0.3.31] — 2026-10-04

Compare:

- Keyboard caret motion onto a red/green change line parks the other pane on the same hunk (same as click).

## [0.3.30] — 2026-10-04

Compare:

- Click a red/green change line to park the other pane on the same hunk ordinal.
- Next/Prev/First/Last Difference status shows both sides (`L12 | R15 (2/5)`).

## [0.3.29] — 2026-10-04

Compare:

- **View → Ignore Whitespace Differences** / **Ignore Case Differences** toggle (✓ in menu), persist to settings, and re-diff immediately while Compare is on.
- Starting Compare parks **both** panes on the first hunk ordinal (not only the left caret).
- Status line shows active ignore flags (`· ignore ws`, `· ignore case`, or `· ignore ws+case`).

## [0.3.28] — 2026-10-04

Compare:

- **View → Swap Compare Sides** flips the left/right pair (scroll included), keeps focus on the new left, and re-diffs so delete/insert colours stay oriented.

## [0.3.27] — 2026-10-04

Compare:

- Closing a tab that is not part of the compare pair remaps both sides so Compare stays on; closing either compared tab still clears Compare.

## [0.3.26] — 2026-10-04

Compare:

- **View → First Difference** / **Last Difference** (⌘/Ctrl+F7 / ⌘/Ctrl+Shift+F7) jump to the first or last change hunk; other pane stays on the same ordinal.

## [0.3.25] — 2026-10-04

Compare:

- Next/Previous Difference also parks the other pane on the same hunk ordinal.
- Status says `(identical)` when both sides match (including ignore options); live re-diff keeps hunk count.

## [0.3.24] — 2026-10-04

Compare:

- Preferences: **Ignore case differences** (`compare_ignore_case` in settings.json). Combines with ignore-whitespace; live re-diff while Compare is on.

## [0.3.23] — 2026-10-04

Compare:

- Starting Compare jumps the left caret to the first change hunk and shows hunk count in the status line.
- Next/Previous Difference status shows hunk ordinal, e.g. `(2/5)`.

## [0.3.22] — 2026-10-04

Project panel:

- **Reveal** opens the workspace folder (or a right-clicked entry) in the OS file manager.
- **Refresh** reloads a cached folder listing instead of re-reading every frame.
- Context menu: Open / Enter folder / Reveal in file manager.

## [0.3.21] — 2026-10-04

Folding:

- Fold margin click uses the same region pick as View → Fold/Unfold Current: toggle works on header markers and on any line inside a foldable block (primary and dual view).

## [0.3.20] — 2026-10-03

Compare:

- **Next / Previous Difference** (View menu, **F7** / **Shift+F7**) jumps the focused pane to the start of the next/previous change hunk while Compare is on (wraps). Mid-hunk Next skips to the following hunk.

## [0.3.19] — 2026-10-03

Settings / shortcuts:

- **Word wrap** is remappable: Preferences → Word wrap shortcut, or `shortcut_word_wrap` in `npp-rs/settings.json` (default `Alt+Z`). Shortcut Mapper and About show the effective binding. Other keys stay hard-wired; full `shortcuts.xml` remap is still later.

## [0.3.18] — 2026-10-03

Undo:

- Multi-caret / column typing coalesce: successive inserts at the same carets merge into one undo unit (same 1s window as plain typing). Replace-selection and deletes still start a new unit.

## [0.3.17] — 2026-10-03

Encoding:

- Open detects **UTF-16 LE/BE without a BOM** when at least two-thirds of 16-bit units have a zero high byte (typical ASCII / Latin). Status notes the missing BOM; the next save writes one.
- Valid UTF-8 without that NUL pattern is unchanged. Invalid UTF-8 still falls back to Windows-1252.

## [0.3.16] — 2026-10-03

Find / replace:

- Opening Find or Replace with a **multi-line selection** turns **Sel** on and searches that range (query stays as-is). A short single-line selection still fills the query.
- Match count is 0 while Sel is on but no range is captured (no silent whole-file search).
- Wrap-off Next/Prev says `passed end of selection` when Sel is on.

## [0.3.15] — 2026-10-03

Find / replace:

- **Re** on the Find bar (and Preferences **Regular expression**) treats the query as a regex. Engine is linear-time, so nested quantifiers do not hang the UI. Invalid patterns report `Find: invalid regex`. Replace uses a literal replacement string.
- Setting `find_regex` in `npp-rs/settings.json`

## [0.3.14] — 2026-10-03

Find / replace:

- **Wrap** on the Find bar (and Preferences) turns wrap-around on or off. Off: Next/Prev stop at the bound (`Find: passed end of file`). Default on, same as before.
- Setting `find_wrap` in `npp-rs/settings.json`

## [0.3.13] — 2026-10-03

Find / replace:

- **In selection** (`Sel` on the Find bar, Preferences Find) limits Next / Prev / Replace All to a captured range
- Setting `find_in_selection` in `npp-rs/settings.json`

## [0.3.12] — 2026-08-31

Issue #14 (P1 lexer-aware folding + fold margin):

- Gutter fold markers (`−` / `+`); click toggles a region
- Brace folds for Rust/C-like languages; indent folds for Python and others
- View → Fold / Unfold / levels use the same regions
- Preferences: **Show fold margin** (`show_fold_margin` in `npp-rs/settings.json`)
- Docs: `docs/folding.md`

## [0.3.11] — 2026-08-31

Issue #13 (P1 autosave / backup-on-save):

- Preferences → Files: **Backup on save** copies the on-disk file into `npp-rs/backup/` (path layout) before overwrite
- Preferences → Files: **Autosave interval** (0 = off, else 15–900s) saves dirty tabs that already have a path
- Settings keys: `backup_on_save`, `autosave_interval_secs` in `npp-rs/settings.json`
- Docs: `docs/autosave-backup.md`

## [0.3.10] — 2026-08-31

Issue #12 (P1 deeper Find in Files):

- Search uses the Project / workspace root and walks folders recursively
- Skips hidden dirs, symlinks, binary / huge files; caps matches and depth
- Include / exclude globs on the Find bar (persisted); default exclude skips `target`, `node_modules`, and similar
- Menu **Search → Find in Files** and the Find bar **Find in Files** button

## [0.3.9] — 2026-08-31

Issue #11 (P0 column / rect select + multi-caret typing):

- Alt+drag (Option+drag on macOS) builds a rectangular / column selection
- Typing, Backspace, Delete, Paste, Enter, and Tab apply to all multi-carets (one undo)
- Copy/Cut of multi ranges joins line slices with newlines
- Docs: `docs/column-mode.md` (no virtual space; arrows clear multi-carets)

## [0.3.8] — 2026-08-31

Issue #10 (P0 file drop + selection drag):

- Drop files onto the window to open them (folders skipped)
- Drag selected text to move; Ctrl/Cmd+drag to copy (one undo)
- Orange drop caret while dragging; dual-view panes supported

## [0.3.7] — 2026-08-31

Issue #9 (P0 hotkeys / Find next):

- F3 / Shift+F3 find next/prev without the Find bar open
- Cmd/Ctrl+G / Shift+G find next/prev are global
- Cmd/Ctrl+L go to line; F2 / Shift+F2 / Cmd+F2 bookmarks
- Cmd/Ctrl+= − 0 and Cmd/Ctrl+mouse wheel zoom; Alt+Z word wrap
- Cmd/Ctrl+H replace; Shortcut Mapper and About lists updated

## [0.3.6] — 2026-08-31

Issue #8 (P1 encoding):

- Open UTF-16 LE/BE with BOM; decode to Unicode in the buffer
- Save as UTF-16 LE/BE when Format (or Convert to) sets that encoding
- Status / docs: `docs/encoding.md`

## [0.3.5] — 2026-08-31

Issue #8 (P1 change-history depth):

- Gutter: full-height SC_MARK_BAR-style blocks (amber unsaved / green saved) with joined runs
- Soft line wash + dual-view pane marks
- Undo/redo remaps line marks via buffer line-structure hooks (no caret re-stamp)
- Status `CHG u/s`, Clear/jump messages name unsaved vs saved

## [0.3.4] — 2026-08-30

Issue #8 (P1 themes depth):

- Theme JSON: selection, caret, whitespace, indent guide, and syntax `tokens` map
- Notepad++ XML subset: GlobalStyles chrome + preferred lexer WordsStyle → highlight tokens
- Samples: deeper `themes/slate.json`, `themes/mini-dark.xml`
- Primary and secondary panes use theme chrome colours


## [0.3.3] — 2026-08-30

- Compare: default partner is the tab to the right (or left if last); ⌘/Ctrl-click or tab menu to pick any second tab
- Help / About / cmdline links use `main` (not `dev`)
- README feature-tour GIF (`docs/screens/`) + rebuild scripts

## [0.3.2] — 2026-08-29

Issue #8 (partial P1):

- Project panel: name filter, Refresh, persist workspace root in settings.json
- ANSI (Windows-1252) save: confirm dialog when characters would become `?`

## [0.3.1] — 2026-08-29

Issue #7 P0 gap batch:

- Preferences: gutter extra, caret blink, default EOL, recent count, restore session, find options, compare ignore-whitespace
- Find/Replace: Match case / Whole word, live match count, persisted query; Replace All is one undo + marks compare stale
- Session: config-dir `npp-rs/session.txt`; opt-in restore on launch; save on quit
- Compare: sync H+V scroll on start; optional ignore whitespace

## [0.3.0] — 2026-08-29

Stability and architecture batch for issues #3 / #4 / #6:

- Atomic saves; DocumentId async loads; tail worker; bookmark remap
- Read-only edit gate; reload + saved revision; transactional undo
- Honest UTF-8/Windows-1252 open; CI fmt/clippy; highlight viewport
- Theme apply MVP; project file panel; RTL layout cue


## [0.2.14] — 2026-08-29

### Stability (issue #4)

- File → Reload from Disk replaces buffer contents (dirty confirm)
- Dirty follows `saved_generation` vs `edit_generation` (undo-to-saved clears dirty)


## [0.2.13] — 2026-08-29

### Stability (issue #4)

- File → Reload from Disk replaces buffer contents (dirty confirm)
- Dirty follows saved vs edit generation (undo-to-saved clears dirty)

### Themes / project / RTL

- Theme MVP: built-in Dark/Light + `themes/*.json`; Preferences and Import Style Theme(s) apply egui visuals and editor bg/fg/gutter
- Project panel lists the workspace folder and opens files on click; Open Folder as Workspace sets the root
- RTL/LTR mirrors editor line anchors and shows an RTL status cue

## [0.2.12] — 2026-08-29

### Read-only

- Menu mutate paths refuse edits when the document is read-only or still loading (`Document::try_buffer_mut`, command `ensure_editable`)
- Toggle / clear read-only commands still work


## [0.2.11] — 2026-08-29

### Build / CI

- Fix TailMsg test initializers, `EditorState::workspace_root`, and edit `ensure_editable` gate
- Restore clippy allows on buffer `from_str` / `to_string`
- Mark issue #4 Agent E checklist done

## [0.2.10] — 2026-08-29

### Encoding honesty

- Open/tail never insert U+FFFD via `from_utf8_lossy`; invalid UTF-8 uses Windows-1252 with clear status
- Tests cover invalid byte sequences (`docs/encoding.md`)

## [0.2.9] — 2026-08-29

### Undo

- One user command is one undo unit (`with_transaction` for multi-edit helpers)
- Typing coalesce: same kind (insert), adjacent caret, within 1s
- Notes: `docs/undo-transactions.md`

## [0.2.8] — 2026-08-29

### Highlight

- Byte→char conversion is one forward pass (no per-span full rescans)
- Refresh uses a viewport-oriented window (still capped at 512 KiB)
- Notes: `docs/highlight-viewport.md`

## [0.2.7] — 2026-08-29

### Bookmarks

- Bookmarks (and other line marks) shift when inserts or deletes change line structure
- Buffer records `LineStructureEdit`; the editor prefers that over the snap heuristic

## [0.2.6] — 2026-08-29

### Stability (issue #4)

- Log tail: disk reads and rotate reload run on a background worker; the UI only applies `TailMsg` (dirty/suspend policy unchanged)

### Tabs / open

- Async large-file load binds to a stable `DocumentId` (not tab index)
- Pending load apply drops when the id is gone or no longer the loading placeholder

## [0.2.5] — 2026-08-29

### Save

- Atomic save: write a sibling temp file, `sync_all`, then rename over the target (std rename replaces on Windows)
- Save no longer creates missing parent directories

## [0.2.4] — 2026-08-29

### CLI

- `-V` / `--version`, `-n` / `--line`, `-ro` / `--read-only`, `--` end of options
- Help → Command Line Arguments documents the same flags

### Preferences follow-up

- View → Word wrap writes `settings.json`
- Edit → Indent Tab uses Preferences tab width

## [0.2.3] — 2026-08-29

### Dual view

- Menu Edit (and Format) commands use the focused pane tab, not only the tab-bar active document

### Encoding

- Format → ANSI: save writes Windows-1252 (lossy); UTF-8 / UTF-8-BOM set per-tab save encoding
- Open files keep the detected encoding for later save (`docs/encoding.md`)

### Compare

- Re-diff line tags after edits (~200 ms debounce) while Compare is on

### Change history

- Amber gutter ticks for unsaved edits; green after save (promote, not clear)
- Line-index remap on insert/delete (`LineEditSnap` / `prepare_edit`)

## [0.2.2] — 2026-08-29

### Release checkpoint

- About tagline: “a Notepad++ inspired editor, rebuilt for fun”
- Overnight gap loop started on issue #6 (`docs/overnight-gaps.md`)

## [0.2.1] — 2026-08-29

### Compare fix

- Move to Other View no longer undoes itself (removed extra Switch)
- Compare pins left/right panes to the compared pair so colours stay visible
- Switch in compare mode swaps both sides and their tags
- Clearer compare how-to in `docs/compare.md`

## [0.2.0] — 2026-08-29

Large feature batch after 0.1.2 (menu parity, dual view, compare, UX).

### Dual view edit

- Other view pane is writable (type, delete, clipboard, caret, selection)
- Click a pane to focus keyboard input; undo shortcuts follow the focused pane
- Sync scroll / zoom sync / compare washes unchanged

### Built-in compare

- View → Compare with Other View: 2-way side-by-side colours (red delete / green insert) + sync scroll
- In-process line LCS (`diff.rs`); Clear Compare to exit; max 3000 lines/side

### Menu parity (issue #1)

- File: `IDM_PINTAB` toggles active-tab pin; Close All but Pinned reports keep/closed counts
- Fold / hide lines: hidden lines leave the viewport
- Style-mark washes and bookmark ticks in the gutter
- Search: style mark, jump, clear, and copy-styled helpers
- Edit Cut / Copy / Paste use the session clipboard
- Placeholder / status-only menus cleared; Coming Soon stubs gone (`docs/menu-todo.md`)

### Editor UX

- Drag tabs on the tab bar to reorder (live slide; dual/compare indices remap)
- Prompt before closing unsaved tabs (Save / Don't Save / Cancel), including Close All variants, Exit, and window close
- Keep the last editor line clear of the status bar (status panel before editor + bottom padding)
- Help (?) → Changelog opens `docs/changelog.md` on GitHub
- Status bar shows `v{version}` and git short hash
- Preferences: log-tail, font size, line numbers
- CLI: open path args + `-h` / `--help`

### Earlier on `dev` (after 0.1.2)

- Command split by domain for parallel agents (`docs/agent-parallel.md`)
- Log open dialog, Help → Debug Info / Open Logs, dirty-safe tail stability

## [0.1.2] — 2026-08-28

### Menu parity (issue #1)

- Encoding menu: all `IDM_FORMAT_*` acknowledged (UTF-8 in memory)
- Edit: sort lines, remove dups, blank below, sentence case, split, Mac EOL

Ready ~337 / stub ~237 — see `docs/menu-todo.md`.

## [0.1.1] — 2026-08-28

### Progress on menu parity ([#1](https://github.com/raro42/npp-rust/issues/1))

- Inventory: `docs/menu-todo.md`
- File: Save All / Copy As / Rename / close variants / open folder & viewer / shell
- Edit: case, join/move lines, datetime, path copy helpers
- Search: set-and-find, Go to Line
- View: zoom, always-on-top, tab switch

## [0.1.0] — 2026-08-28

First public release of **npp-rs**.

### Features

- Multi-tab editor (UTF-8), open / save / recent files
- Find and replace
- Undo / redo, indent, line ops, word select
- Tree-sitter highlight (Rust, C/C++, Python, SQL, Markdown, JSON, …)
- Full Notepad++-style menu tree; ready items tinted teal; stubs show Coming Soon
- In-process plugins and format helpers
- Agent loop + public-repo privacy gates (`agents/`, `scripts/gh-safe.sh`)

### Builds

GitHub Actions release binaries for Linux, Windows, and macOS.
