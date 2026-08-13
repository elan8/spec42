# META
~~~ini
description=KerML Association: ProductSelection_UnownedEnds
type=file
~~~
# SOURCE
~~~kerml
package ProductSelection_UnownedEnds {
	
	class SelectionInfo;
	class ShoppingCart {
		feature selectedProducts : Product[0..*];
	}
	class Product {
		feature inCart: ShoppingCart[0..1];
	}
	
	assoc ProductSelection {
		feature info: SelectionInfo[1];
		
		end feature cart: ShoppingCart[1] crosses selectedProduct.inCart;
		end feature selectedProduct: Product[1] crosses cart.selectedProducts;
	}
	
	assoc SingleProductSelection :> ProductSelection {
		end feature cart: ShoppingCart[1];
		end [0..1] feature selectedProduct: Product[1];
	}
	
	// Equivalent association showing implied relationships explicitly.
	assoc SingleProductSelection1 :> ProductSelection {
		end feature cart: ShoppingCart[1] redefines cart {
			public import selectedProduct::selectedProduct1;
		}
		end feature selectedProduct: Product[1] redefines selectedProduct crosses cart.selectedProduct1 {
			member feature selectedProduct1[0..1] subsets ShoppingCart::selectedProducts featured by ShoppingCart;
		}
	}
	
	class OnlineCustomer {
		feature info1: SelectionInfo;	
		feature myCart: ShoppingCart[1];	
		feature products: Product[0..*];
		
		connector ps1 : ProductSelection from myCart to products {
			:>> info = info1;
		}
		
		connector ps2 : ProductSelection from [1] myCart to [1] products {
			:>> info = info1;
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/product_selection_unowned_ends.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 4 2) (end 5 1))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 7 2) (end 8 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 10 1) (end 15 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 10 1) (end 15 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 17 1) (end 20 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 17 1) (end 20 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 23 1) (end 30 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 23 1) (end 30 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 33 2) (end 34 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 34 2) (end 35 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 35 2) (end 37 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 37 2) (end 41 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 41 2) (end 44 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:88c3612c9711c48da83f3ee410c82a7a35f2cad03b830bb9bd4eaa4ecb57ab0e") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::OnlineCustomer"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::Product"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SelectionInfo"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart"))) (kind class-def) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
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
