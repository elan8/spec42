import * as vscode from "vscode";

export class SysmlReferencePanel {
  private static current: SysmlReferencePanel | undefined;
  private readonly panel: vscode.WebviewPanel;

  static open(context: vscode.ExtensionContext): void {
    if (SysmlReferencePanel.current) {
      SysmlReferencePanel.current.panel.reveal();
      return;
    }
    const panel = vscode.window.createWebviewPanel(
      "spec42.sysmlReference",
      "SysML v2 Quick Reference",
      vscode.ViewColumn.Active,
      { enableScripts: false, retainContextWhenHidden: true }
    );
    SysmlReferencePanel.current = new SysmlReferencePanel(panel, context);
  }

  private constructor(panel: vscode.WebviewPanel, _context: vscode.ExtensionContext) {
    this.panel = panel;
    this.panel.webview.html = this.buildHtml();
    this.panel.onDidDispose(() => {
      SysmlReferencePanel.current = undefined;
    });
  }

  private buildHtml(): string {
    return `<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'unsafe-inline';">
<title>SysML v2 Quick Reference</title>
<style>
  body { font-family: var(--vscode-font-family); font-size: var(--vscode-font-size); color: var(--vscode-editor-foreground); background: var(--vscode-editor-background); margin: 0; padding: 24px 32px; max-width: 860px; }
  h1 { font-size: 20px; font-weight: 600; margin: 0 0 4px; }
  .subtitle { color: var(--vscode-descriptionForeground); margin: 0 0 28px; font-size: 13px; }
  h2 { font-size: 13px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.6px; color: var(--vscode-descriptionForeground); border-bottom: 1px solid var(--vscode-panel-border); padding-bottom: 6px; margin: 28px 0 12px; }
  table { width: 100%; border-collapse: collapse; }
  td { padding: 6px 8px; vertical-align: top; border-bottom: 1px solid var(--vscode-panel-border); font-size: 13px; line-height: 1.5; }
  td:first-child { white-space: nowrap; width: 1%; }
  tr:last-child td { border-bottom: none; }
  code { font-family: var(--vscode-editor-font-family, monospace); font-size: 12px; background: var(--vscode-textCodeBlock-background); border-radius: 3px; padding: 1px 5px; }
  pre { font-family: var(--vscode-editor-font-family, monospace); font-size: 12px; background: var(--vscode-textCodeBlock-background); border-radius: 5px; padding: 12px 14px; margin: 10px 0; overflow-x: auto; line-height: 1.55; }
  .keyword { color: var(--vscode-symbolIcon-keywordForeground, #569cd6); }
  .comment { color: var(--vscode-symbolIcon-colorForeground, #6a9955); font-style: italic; }
  .string { color: var(--vscode-symbolIcon-stringForeground, #ce9178); }
  .note { background: var(--vscode-textBlockQuote-background); border-left: 3px solid var(--vscode-textBlockQuote-border); padding: 8px 12px; border-radius: 0 4px 4px 0; font-size: 12px; color: var(--vscode-descriptionForeground); margin: 12px 0; }
</style>
</head>
<body>
<h1>SysML v2 Quick Reference</h1>
<p class="subtitle">Core concepts for Spec42 / SysML v2. See the <a href="https://elan8.github.io/spec42/" style="color: var(--vscode-textLink-foreground);">Spec42 docs</a> or the <a href="https://www.omg.org/spec/SysML/2.0/Language/" style="color: var(--vscode-textLink-foreground);">OMG SysML v2 Language Specification</a>.</p>

<h2>Definitions</h2>
<p class="note">Definitions declare reusable types. Usages instantiate them in context. Every usage keyword has a matching <code>def</code> form.</p>
<table>
  <tr><td><code>part def</code></td><td>A structural component type. Parts compose a system hierarchy.</td></tr>
  <tr><td><code>item def</code></td><td>Things that flow between parts (messages, fuel, signals). Supertype of <code>part def</code>.</td></tr>
  <tr><td><code>attribute def</code></td><td>A data value type (scalar, string, quantity). Always referential — no sub-parts.</td></tr>
  <tr><td><code>enum def</code></td><td>An attribute definition with a fixed set of enumeration literals.</td></tr>
  <tr><td><code>port def</code></td><td>A typed interaction point through which parts connect or flow.</td></tr>
  <tr><td><code>action def</code></td><td>A behavioral step or function a part can perform.</td></tr>
  <tr><td><code>state def</code></td><td>A state machine with entry/do/exit actions and transitions.</td></tr>
  <tr><td><code>connection def</code></td><td>A typed binary link between two end features (ports or parts).</td></tr>
  <tr><td><code>allocation def</code></td><td>A binary mapping from a logical element to a physical one.</td></tr>
  <tr><td><code>requirement def</code></td><td>A formal requirement with a subject, text body, and constraints.</td></tr>
  <tr><td><code>view def</code></td><td>A specialized <code>part def</code> that selects and renders a model subset.</td></tr>
  <tr><td><code>metadata def</code></td><td>A structured annotation type applied to elements with <code>@</code>.</td></tr>
</table>

<h2>Usages</h2>
<p class="note">Drop the <code>def</code> suffix to get the usage keyword. Usages appear inside definitions and create owned, typed members.</p>
<table>
  <tr><td><code>part</code></td><td>A structural member inside a containing part or package.</td></tr>
  <tr><td><code>item</code></td><td>Something that flows or is transferred (payload, message, entity).</td></tr>
  <tr><td><code>attribute</code></td><td>A data property on a definition or usage, always referential.</td></tr>
  <tr><td><code>port</code></td><td>An interaction point on a part, typed by a <code>port def</code>.</td></tr>
  <tr><td><code>action</code></td><td>A behavioral step: a usage of an <code>action def</code>.</td></tr>
  <tr><td><code>state</code></td><td>A state usage inside a state machine body.</td></tr>
  <tr><td><code>connection</code></td><td>Instantiates a <code>connection def</code> between two end features.</td></tr>
  <tr><td><code>allocate</code></td><td>Shorthand for an allocation usage: <code>allocate source to target;</code></td></tr>
  <tr><td><code>satisfy requirement</code></td><td>Asserts that a design element satisfies a named requirement.</td></tr>
  <tr><td><code>expose</code></td><td>Imports elements into a view (like <code>import</code>, but always protected).</td></tr>
</table>

<h2>Relationships &amp; Symbols</h2>
<table>
  <tr><td><code>:&gt;</code></td><td><strong>Specialization / subsetting.</strong> On definitions: inherits and refines a parent type. On usages: subsets an inherited member.<br><code>part def Truck :&gt; Vehicle { ... }</code><br><code>part engine :&gt; powerUnit;</code></td></tr>
  <tr><td><code>:&gt;&gt;</code></td><td><strong>Redefinition.</strong> A usage overrides (redefines) an inherited member — narrower than subsetting.<br><code>part redefines cylinders[4];</code><br><code>attribute :&gt;&gt; mass = 12 [kg];</code></td></tr>
  <tr><td><code>::&gt;</code></td><td><strong>Reference subsetting.</strong> An end feature references (points to) an existing usage without owning it. Used mainly on connection ends.<br><code>end part hub ::&gt; mainSwitch;</code></td></tr>
  <tr><td><code>:</code></td><td><strong>Typing.</strong> A usage is classified by a definition.<br><code>part w : Wheel;</code></td></tr>
  <tr><td><code>bind … = …</code></td><td><strong>Binding connector.</strong> Declares that two features always have equal values.<br><code>bind fuelTank.fuelFlowOut = engine.fuelFlowIn;</code></td></tr>
  <tr><td><code>= &lt;expr&gt;</code></td><td><strong>Feature value.</strong> Assigns a fixed or default value to a feature (not a connector).<br><code>attribute mass = 1200 [kg];</code></td></tr>
</table>

<h2>Annotations &amp; Metadata</h2>
<table>
  <tr><td><code>@</code></td><td>Applies a <code>metadata def</code> to an element: <code>@ Approved { by = "Jane"; }</code></td></tr>
  <tr><td><code>doc</code></td><td>Attaches a documentation string: <code>doc /* Human-readable description */</code></td></tr>
  <tr><td><code>abstract</code></td><td>Marks a definition or usage as abstract (cannot be instantiated directly).</td></tr>
  <tr><td><code>variation</code></td><td>Marks a definition whose members are mutually exclusive variant choices.</td></tr>
</table>

<h2>Views &amp; Visualization</h2>
<table>
  <tr><td><code>view def</code></td><td>Defines a kind of view with filter conditions and a rendering rule.</td></tr>
  <tr><td><code>view</code></td><td>A usage of a <code>view def</code> that selects concrete elements to expose.</td></tr>
  <tr><td><code>expose</code></td><td>Selects elements into a view (supports <code>::*</code> wildcard and <code>::**</code> recursive).</td></tr>
  <tr><td><code>filter</code></td><td>Boolean condition limiting which elements are included: <code>filter not @SysML::ConnectionUsage;</code></td></tr>
  <tr><td><code>render</code></td><td>Specifies the rendering tool/format for the view: <code>render asTreeDiagram;</code></td></tr>
</table>

<h2>Example: minimal model</h2>
<pre><span class="keyword">package</span> MySystem {

    <span class="keyword">attribute def</span> Mass { <span class="keyword">attribute</span> :&gt;&gt; num; <span class="keyword">attribute</span> :&gt;&gt; mRef = 1 [kg]; }

    <span class="keyword">part def</span> Engine {
        <span class="keyword">attribute</span> mass : Mass;
        <span class="keyword">port</span> powerOut;
    }

    <span class="keyword">part def</span> Vehicle :&gt; Engine {   <span class="comment">// specializes Engine</span>
        <span class="keyword">part</span> engine : Engine;
        <span class="keyword">attribute</span> :&gt;&gt; mass = 1200 [kg];  <span class="comment">// redefines + sets value</span>
        <span class="keyword">bind</span> engine.powerOut = powerOut;  <span class="comment">// binding connector</span>
    }

    <span class="keyword">view def</span> Overview {
        <span class="keyword">expose</span> Vehicle::**;          <span class="comment">// all nested elements, recursively</span>
        <span class="keyword">filter</span> <span class="keyword">not</span> @SysML::AttributeUsage;
    }
}</pre>

</body>
</html>`;
  }
}
