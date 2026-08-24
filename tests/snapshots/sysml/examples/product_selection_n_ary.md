# META
~~~ini
description=SysML Example (Association): ProductSelection_N_ary
type=file
~~~
# SOURCE
~~~sysml
package ProductSelection_N_ary_SysML {
	
	item def ShoppingCart;
	item def Product;
	item def Account;
	
	// User-specified connection defiation definition
	connection def ProductSelection {
		end [0..1] item cart: ShoppingCart[1];
		end [0..*] item selectedProduct: Product[1];
		end [1..1] item account : Account[1];
	}
	
	// Equivalent connection defiation definition with named end items.
	connection def ProductSelection1 {
		end inCart[0..1] item cart: ShoppingCart[1];
		end selectedProducts[0..*] item selectedProduct: Product[1];
		end withAccount[1..1] item account : Account[1];
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/product_selection_n_ary.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:68fd1a01fb1408ea31d229e6126320f11767c33b7be4b5ea375dafb22680226d") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Account"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Product"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::account"))) (kind item) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Account")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::account::withAccount"))) (kind ref) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::cart"))) (kind item) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::cart::inCart"))) (kind ref) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::selectedProduct"))) (kind item) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::selectedProduct::selectedProducts"))) (kind ref) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::account"))) (kind item) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Account")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary_SysML")) (named (kind connection-def) (name "ProductSelection")) (named (kind item) (name "account")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::cart"))) (kind item) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary_SysML")) (named (kind connection-def) (name "ProductSelection")) (named (kind item) (name "cart")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::selectedProduct"))) (kind item) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary_SysML")) (named (kind connection-def) (name "ProductSelection")) (named (kind item) (name "selectedProduct")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart"))) (kind item-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::account"))) (kind featureTyping) (ordinal 0))
      (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Account")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::selectedProduct"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::account"))) (kind featureTyping) (ordinal 0))
      (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Account")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Product")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::account"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Account"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::account"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::cart"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::selectedProduct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::account"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Account"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::account"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::cart"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::account"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::account::withAccount"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::account"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::cart"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::cart::inCart"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::cart"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::selectedProduct::selectedProducts"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::selectedProduct"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::account"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary_SysML")) (named (kind connection-def) (name "ProductSelection")) (named (kind item) (name "account")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::account"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::cart"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary_SysML")) (named (kind connection-def) (name "ProductSelection")) (named (kind item) (name "cart")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::cart"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary_SysML")) (named (kind connection-def) (name "ProductSelection")) (named (kind item) (name "selectedProduct")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::selectedProduct"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Account")))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::account")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::account")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Product")))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::selectedProduct")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::selectedProduct")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::account")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Account")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Account")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Account")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::account::withAccount")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::account")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::cart")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::cart::inCart")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::cart")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::selectedProduct")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Product")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Product")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Product")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::selectedProduct::selectedProducts")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::selectedProduct")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::account")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Account")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Account")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Account")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary_SysML")) (named (kind connection-def) (name "ProductSelection")) (named (kind item) (name "account")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::account")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::cart")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary_SysML")) (named (kind connection-def) (name "ProductSelection")) (named (kind item) (name "cart")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::cart")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::selectedProduct")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Product")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Product")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Product")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary_SysML")) (named (kind connection-def) (name "ProductSelection")) (named (kind item) (name "selectedProduct")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::selectedProduct")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart")))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::cart")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::cart")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 17 39) (end 17 46)) (probe (position 17 39))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::account"))) (kind featureTyping) (ordinal 0) (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Account")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 15 30) (end 15 42)) (probe (position 15 30))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 16 51) (end 16 58)) (probe (position 16 51))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::selectedProduct"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Product")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 10 28) (end 10 35)) (probe (position 10 28))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::account"))) (kind featureTyping) (ordinal 0) (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Account")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 8 24) (end 8 36)) (probe (position 8 24))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 9 35) (end 9 42)) (probe (position 9 35))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Product")))))
    )
  )
)
~~~
