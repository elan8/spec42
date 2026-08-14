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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:68fd1a01fb1408ea31d229e6126320f11767c33b7be4b5ea375dafb22680226d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Account"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Product"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::inCart"))) (kind connection) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::selectedProducts"))) (kind connection) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::withAccount"))) (kind connection) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::account"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Account"))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::cart"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart"))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::selectedProduct"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product"))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart"))) (kind item-def) (membership (kind owning) (visibility default)))
  )
  (references
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
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::account"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Account"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::account"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::cart"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 10 28) (end 10 35)) (probe (position 10 28))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::account"))) (kind featureTyping) (ordinal 0) (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Account")))))
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 8 24) (end 8 36)) (probe (position 8 24))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart")))))
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 9 35) (end 9 42)) (probe (position 9 35))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Product")))))
  )
)
~~~
