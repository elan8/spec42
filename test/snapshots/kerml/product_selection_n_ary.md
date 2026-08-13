# META
~~~ini
description=KerML Association: ProductSelection_N_ary
type=file
~~~
# SOURCE
~~~kerml
package ProductSelection_N_ary {
	
	class ShoppingCart;
	class Product;
	class Account;
	
	// User-specified association definition
	assoc ProductSelection {
		end [0..1] feature cart: ShoppingCart[1];
		end [0..*] feature selectedProduct: Product[1];
		end [1..1] feature account : Account[1];
	}
	
	// Equivalent association definition with named end features.
	assoc ProductSelection1 {
		end inCart[0..1] feature cart: ShoppingCart[1];
		end selectedProducts[0..*] feature selectedProduct: Product[1];
		end withAccount[1..1] feature account : Account[1];
	}
	
	// Equivalent association definition with nested cross features.
	assoc ProductSelection2 {
		end feature cart: ShoppingCart[1] {
			member feature inCart[0..1]; // owned cross feature
		}
		end feature selectedProduct: Product[1] {
			member feature selectedProducts[0..*]; // owned cross feature
		}
		end feature account : Account[1] {
			member feature withAccount[1..1]; // owned cross feature
		}
	}
	
	// Equivalent association definition showing library model specialization 
	// implied cross subsetting, and "Cartesian product" features.
	assoc ProductSelection3 specializes Links::Link {
		end cart: ShoppingCart[1] crosses cart::product_account.inCart {
			member feature inCart: ShoppingCart[0..1] featured by Product_Account {
                // Represents the "Cartesian product" of Product X Account.
				member feature Product_Account : Account featured by Product;
			}
			member feature product_account : inCart::Product_Account featured by ProductSelection3 {
				public import inCart;
			}
		}
		end selectedProduct: Product[1] crosses selectedProduct::cart_account.selectedProducts {
			member feature selectedProducts: Product[0..*] featured by Cart_Account {
                // Represents the "Cartesian product" of ShoppingCart X Account.
				member feature Cart_Account : Account featured by ShoppingCart;				
			}
			member feature cart_account : selectedProducts::Cart_Account featured by ProductSelection3 {
				public import selectedProducts;
			}
		}
		end feature account : Account[1] crosses account::cart_product.withAccount {
			member feature withAccount[1..1] : Account featured by Cart_Product {
                // Represents the "Cartesian product" of ShoppingCart X Product.
				member feature Cart_Product : Product featured by ShoppingCart;
			}
			member feature cart_product : withAccount::Cart_Product featured by ProductSelection3 {
				public import withAccount;
			}
		}
	}
	
	assoc SingleProductSelection specializes ProductSelection {
		end [0..1] feature cart: ShoppingCart[1];
		end [0..1] feature selectedProduct: Product[1];
		end [1..1] feature account : Account[1];
	}

	assoc SingleProductSelection1 specializes ProductSelection1 {
		end inCart1 [0..1] feature cart: ShoppingCart[1];
		end selectedProduct1 [0..1] feature selectedProduct: Product[1];
		end withAccount1 [1..1] feature account : Account[1];
	}
	
	assoc SingleProductSelection2 specializes ProductSelection2 {
		end feature cart: ShoppingCart[1] {
			member feature inCart1[0..1]; // owned crossing feature
		}
		end feature selectedProduct: Product[1] {
			member feature selectedProducts1[0..*]; // owned crossing feature
		}
		end feature account : Account[1] {
			member feature withAccount1[0..*]; // owned crossing feature
		}
	}
	
	assoc SingleProductSelection3 specializes ProductSelection3 {
		end cart: ShoppingCart[1] redefines cart crosses cart::product_account1.inCart1 {
			member feature inCart1: ShoppingCart[0..1] featured by Product_Account1 {
				member feature Product_Account1 subsets Product_Account : Account featured by Product;
			}
			member feature product_account1 : inCart1::Product_Account1 featured by ProductSelection3 {
				public import inCart1;
			}
		}
		end selectedProduct: Product[1] redefines selectedProduct crosses selectedProduct::cart_account1.selectedProduct1 {
			member feature selectedProduct1: Product[1..1] featured by Cart_Account1 {
				member feature Cart_Account1 subsets Cart_Account : Account featured by ShoppingCart;				
			}
			member feature cart_account1 : selectedProduct1::Cart_Account1 featured by ProductSelection3 {
				public import selectedProduct1;
			}
		}
		end feature account : Account[1] crosses account::cart_product1.withAccount1 {
			member feature withAccount1[1..1] : Account featured by cart_product1 {
				member feature Cart_Product1 subsets Cart_Product : Product featured by ShoppingCart;
			}
			member feature cart_product1 : withAccount1::Cart_Product1 featured by ProductSelection3 {
				public import withAccount1;
			}
		}
	}
	
	class OnlineCustomer {
		feature myCart: ShoppingCart[1];	
		feature products: Product[0..*];
		feature myAccount : Account[1];
		
		connector ps1 : ProductSelection (myCart, products, myAccount);
		
		connector ps2 : ProductSelection ([1] myCart, [0..1] products, [1] myAccount);
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/product_selection_n_ary.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 2 1) (end 2 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 3 1) (end 3 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 4 1) (end 4 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 7 1) (end 11 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 7 1) (end 11 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 14 1) (end 18 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 14 1) (end 18 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 21 1) (end 31 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 21 1) (end 31 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 35 1) (end 63 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 35 1) (end 63 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 65 1) (end 69 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 65 1) (end 69 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 71 1) (end 75 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 71 1) (end 75 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 77 1) (end 87 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 77 1) (end 87 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 89 1) (end 114 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 89 1) (end 114 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 116 1) (end 124 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 117 2) (end 118 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 118 2) (end 119 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 119 2) (end 121 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 121 2) (end 123 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 123 2) (end 124 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:13d76f0df00af0ed1c3c3edd4982752d35cd22830e948f987403688f3e3bbb67") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary"))) (kind package) (membership (kind owning) (visibility default)))
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
