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
  (document "product_selection_n_ary.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f816064ec31fdd0eff5ceb3d889c2f3a79c5353aa28fe4a857aecba5be77a893") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary"))) (kind "package") (name "ProductSelection_N_ary") (declared-name "ProductSelection_N_ary"))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary::Account"))) (kind "classifier decl") (name "Account") (declared-name "Account") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary::OnlineCustomer"))) (kind "classifier decl") (name "OnlineCustomer") (declared-name "OnlineCustomer") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary::Product"))) (kind "classifier decl") (name "Product") (declared-name "Product") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary::ProductSelection"))) (kind "kermlDecl") (name "ProductSelection") (declared-name "ProductSelection") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary::ProductSelection1"))) (kind "kermlDecl") (name "ProductSelection1") (declared-name "ProductSelection1") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary::ProductSelection2"))) (kind "kermlDecl") (name "ProductSelection2") (declared-name "ProductSelection2") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary::ProductSelection3"))) (kind "kermlDecl") (name "ProductSelection3") (declared-name "ProductSelection3") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary::ShoppingCart"))) (kind "classifier decl") (name "ShoppingCart") (declared-name "ShoppingCart") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary::SingleProductSelection"))) (kind "kermlDecl") (name "SingleProductSelection") (declared-name "SingleProductSelection") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary::SingleProductSelection1"))) (kind "kermlDecl") (name "SingleProductSelection1") (declared-name "SingleProductSelection1") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary::SingleProductSelection2"))) (kind "kermlDecl") (name "SingleProductSelection2") (declared-name "SingleProductSelection2") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_N_ary::SingleProductSelection3"))) (kind "kermlDecl") (name "SingleProductSelection3") (declared-name "SingleProductSelection3") (parent (node (document "d0") (qualified-name "ProductSelection_N_ary"))))
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
