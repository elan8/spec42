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
      (diagnostic
        (severity error)
        (code "recovered_connection_def_body_element")
        (source "parser")
        (range (start 8 2) (end 9 2))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 8 2) (end 9 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:68fd1a01fb1408ea31d229e6126320f11767c33b7be4b5ea375dafb22680226d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Account"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::Product"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::inCart"))) (kind connection) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1)) (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::selectedProducts"))) (kind connection) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded)) (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::withAccount"))) (kind connection) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1)) (positional-end 2)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart"))) (kind item-def) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1")))
      (positional-ends (authored 3) (effective 3))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::inCart")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::selectedProducts")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::withAccount")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
