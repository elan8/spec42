# META
~~~ini
description=KerML Association: ProductSelection_OwnedEnds
type=file
~~~
# SOURCE
~~~kerml
package ProductSelection_OwnedEnds {
	
	class SelectionInfo;
	class ShoppingCart;
	class Product;
	
	// User-specified association definition
	assoc ProductSelection {
		feature info: SelectionInfo;
		
		end [0..1] feature cart: ShoppingCart[1];
		end [0..*] nonunique feature selectedProduct: Product[1];
	}
	
	// Equivalent association definition with named end features.
	assoc ProductSelection1 {
		feature info: SelectionInfo;
		
		end inCart[0..1] feature cart: ShoppingCart[1];
		end selectedProducts[0..*] feature selectedProduct: Product[1];
	}
	
	// Equivalent association definition with nested cross features.
	assoc ProductSelection2 {
		feature info: SelectionInfo;
		
		end feature cart: ShoppingCart[1] { 
			member feature inCart[0..1]; // owned cross feature
		}
		end feature selectedProduct: Product[1] { 
			member feature selectedProducts[0..*]; // owned cross feature
		}
	}
	
	// Equivalent association definition showing library model specialization 
	// and implied cross subsetting.
	assoc ProductSelection3 specializes Links::BinaryLink {
		feature info: SelectionInfo;
		
		end cart: ShoppingCart[1] redefines source crosses selectedProduct.inCart {
			member feature inCart: ShoppingCart[0..1] featured by Product;
			public import selectedProduct::selectedProducts;
		}
		end selectedProduct: Product[1] redefines target crosses cart.selectedProducts {
			member feature selectedProducts: Product[0..*] featured by ShoppingCart;
			public import cart::inCart;
		}
	}
	
	assoc SingleProductSelection specializes ProductSelection {
		end [0..1] feature cart: ShoppingCart[1];
		end [0..1] feature selectedProduct: Product[1];
	}

	assoc SingleProductSelection1 specializes ProductSelection1 {
		end inCart1 [0..1] feature cart: ShoppingCart[1];
		end selectedProduct1 [0..1] feature selectedProduct: Product[1];
	}
	
	assoc SingleProductSelection2 specializes ProductSelection2 {
		end feature cart: ShoppingCart[1] {
			member feature inCart1[0..1]; // owned crossing feature
		}
		end feature selectedProduct: Product[1] {
			member feature selectedProduct1[0..1]; // owned crossing feature
		}
	}
	
	assoc SingleProductSelection3 specializes ProductSelection3 {
		end cart: ShoppingCart[1] redefines cart crosses selectedProduct.inCart1 {
			member feature inCart1[0..1] subsets inCart featured by Product;
			public import selectedProduct::selectedProduct1;
		}
		end selectedProduct: Product[1] redefines selectedProduct crosses cart.selectedProduct1 {
			member feature selectedProduct1[0..1] subsets selectedProducts featured by ShoppingCart;
			public import cart::inCart1;
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
  (document "memory://snapshot/product_selection_owned_ends.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 7 1) (end 12 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 7 1) (end 12 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 15 1) (end 20 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 15 1) (end 20 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 23 1) (end 32 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 23 1) (end 32 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 36 1) (end 47 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 36 1) (end 47 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 49 1) (end 52 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 49 1) (end 52 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 54 1) (end 57 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 54 1) (end 57 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 59 1) (end 66 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 59 1) (end 66 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 68 1) (end 77 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 68 1) (end 77 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 80 2) (end 81 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 81 2) (end 82 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 82 2) (end 84 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 84 2) (end 88 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 88 2) (end 91 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:bcd09e48bb97b38fe98180c554d872e02a22679552e066dd58a131ab917240a5") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::OnlineCustomer"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SelectionInfo"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart"))) (kind class-def) (membership (kind owning) (visibility default)))
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
