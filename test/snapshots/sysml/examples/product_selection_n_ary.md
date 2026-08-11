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
  (document "product_selection_n_ary.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 2) (end 15 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 2) (end 16 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 2) (end 17 50))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "9bd2b07751acebe2138c06342bc9774a25a4ec843165736d42bd716ca0336131") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML"))) (kind "package") (name "ProductSelection_N_ary_SysML") (declared-name "ProductSelection_N_ary_SysML"))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::Account"))) (kind "item def") (name "Account") (declared-name "Account") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::Product"))) (kind "item def") (name "Product") (declared-name "Product") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection"))) (kind "connection def") (name "ProductSelection") (declared-name "ProductSelection") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1"))) (kind "connection def") (name "ProductSelection1") (declared-name "ProductSelection1") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::inCart"))) (kind "interface end") (name "inCart") (declared-name "inCart") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::selectedProducts"))) (kind "interface end") (name "selectedProducts") (declared-name "selectedProducts") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::withAccount"))) (kind "interface end") (name "withAccount") (declared-name "withAccount") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::account"))) (kind "interface end") (name "account") (declared-name "account") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection"))) (authored (relationships (typing (reference "Account")))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::cart"))) (kind "interface end") (name "cart") (declared-name "cart") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection"))) (authored (relationships (typing (reference "ShoppingCart")))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::selectedProduct"))) (kind "interface end") (name "selectedProduct") (declared-name "selectedProduct") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection"))) (authored (relationships (typing (reference "Product")))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart"))) (kind "item def") (name "ShoppingCart") (declared-name "ShoppingCart") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::inCart"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::selectedProducts"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection1::withAccount"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::account"))) (kind featureTyping) (ordinal 0)) (authored-target "Account") (outcome (status resolved) (target (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::Account")))))
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::cart"))) (kind featureTyping) (ordinal 0)) (authored-target "ShoppingCart") (outcome (status resolved) (target (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart")))))
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0)) (authored-target "Product") (outcome (status resolved) (target (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::Product")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::account"))) (target (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::Account"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::account"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::cart"))) (target (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::selectedProduct"))) (target (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::Product"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ProductSelection_N_ary_SysML::ProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
