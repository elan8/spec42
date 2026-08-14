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
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 35 37) (end 35 48))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 37 3) (end 41 3))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 41 3) (end 44 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 46 3) (end 50 3))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 50 3) (end 53 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 55 3) (end 59 3))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 59 3) (end 62 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 91 3) (end 94 3))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 94 3) (end 97 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 99 3) (end 102 3))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 102 3) (end 105 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 107 3) (end 110 3))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 110 3) (end 113 2))
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
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::OnlineCustomer"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::inCart"))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::inCart::cart"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::selectedProducts"))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::selectedProducts::selectedProduct"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::withAccount"))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::withAccount::account"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Account")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::account"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Account")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::account::withAccount"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers member) (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::cart"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::cart::inCart"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers member) (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::selectedProduct"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::selectedProduct::selectedProducts"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers member) (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3"))) (kind kerml-association) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Links::Link")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::account"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Account")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::cart"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::selectedProduct"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 0))))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 1))))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 2))))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 2)) (named (kind kerml-feature) (name "account"))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Account")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "cart"))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "selectedProduct"))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection"))) (kind kerml-association) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ProductSelection")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1"))) (kind kerml-association) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ProductSelection1")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::inCart1"))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::inCart1::cart"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::selectedProduct1"))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::selectedProduct1::selectedProduct"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::withAccount1"))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::withAccount1::account"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Account")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2"))) (kind kerml-association) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ProductSelection2")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::account"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Account")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::account::withAccount1"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers member) (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::cart"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::cart::inCart1"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers member) (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::selectedProduct"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::selectedProduct::selectedProducts1"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers member) (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3"))) (kind kerml-association) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ProductSelection3")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::account"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Account")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::cart"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart")) (redefinition (reference "cart")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::selectedProduct"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product")) (redefinition (reference "selectedProduct")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 0))))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 1))))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 2))))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 2)) (named (kind kerml-feature) (name "account"))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Account")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "cart"))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart")))))
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "selectedProduct"))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::inCart::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::selectedProducts::selectedProduct"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::withAccount::account"))) (kind featureTyping) (ordinal 0))
      (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::account"))) (kind featureTyping) (ordinal 0))
      (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::selectedProduct"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3"))) (kind specialization) (ordinal 0))
      (authored-target "Links::Link")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::account"))) (kind featureTyping) (ordinal 0))
      (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::selectedProduct"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 2)) (named (kind kerml-feature) (name "account"))))) (kind featureTyping) (ordinal 0))
      (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "cart"))))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "selectedProduct"))))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection"))) (kind specialization) (ordinal 0))
      (authored-target "ProductSelection")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1"))) (kind specialization) (ordinal 0))
      (authored-target "ProductSelection1")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::inCart1::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::selectedProduct1::selectedProduct"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::withAccount1::account"))) (kind featureTyping) (ordinal 0))
      (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2"))) (kind specialization) (ordinal 0))
      (authored-target "ProductSelection2")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::account"))) (kind featureTyping) (ordinal 0))
      (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::selectedProduct"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3"))) (kind specialization) (ordinal 0))
      (authored-target "ProductSelection3")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::account"))) (kind featureTyping) (ordinal 0))
      (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::cart"))) (kind redefinition) (ordinal 0))
      (authored-target "cart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::cart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::selectedProduct"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::selectedProduct"))) (kind redefinition) (ordinal 0))
      (authored-target "selectedProduct")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::selectedProduct")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 2)) (named (kind kerml-feature) (name "account"))))) (kind featureTyping) (ordinal 0))
      (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "cart"))))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "selectedProduct"))))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::inCart::cart"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::inCart::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::selectedProducts::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::selectedProducts::selectedProduct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::withAccount::account"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::withAccount::account"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::account"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::account"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::cart"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::selectedProduct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::account"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::account"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::cart"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::selectedProduct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 2)) (named (kind kerml-feature) (name "account"))))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 2)) (named (kind kerml-feature) (name "account"))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "cart"))))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "cart"))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "selectedProduct"))))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "selectedProduct"))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::inCart1::cart"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::inCart1::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::selectedProduct1::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::selectedProduct1::selectedProduct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::withAccount1::account"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::withAccount1::account"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::account"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::account"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::cart"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::selectedProduct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::account"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::account"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::cart"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::cart"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::cart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::cart"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::selectedProduct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::selectedProduct"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::selectedProduct"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 2)) (named (kind kerml-feature) (name "account"))))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 2)) (named (kind kerml-feature) (name "account"))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "cart"))))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "cart"))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "selectedProduct"))))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "selectedProduct"))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::account"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::account"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::cart"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::cart"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::selectedProduct"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::account"))) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::account"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::withAccount::account")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::account")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::account")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 2)) (named (kind kerml-feature) (name "account")))) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::withAccount1::account")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::account")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::account")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 2)) (named (kind kerml-feature) (name "account")))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::selectedProducts::selectedProduct")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::selectedProduct")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::selectedProduct")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "selectedProduct")))) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::selectedProduct1::selectedProduct")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::selectedProduct")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::selectedProduct")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "selectedProduct")))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection")))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1")))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::inCart")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::inCart::cart")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::inCart")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::selectedProducts")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::selectedProducts::selectedProduct")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::selectedProducts")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::withAccount")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::withAccount::account")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::withAccount")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2")))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::account")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::account")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::account::withAccount")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::account")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::cart")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::cart")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::cart::inCart")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::cart")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::selectedProduct")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::selectedProduct")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::selectedProduct::selectedProducts")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::selectedProduct")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3")))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::account")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::account")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::cart")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::selectedProduct")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 2)) (named (kind kerml-feature) (name "account")))))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 2)))))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "cart")))))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 0)))))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "selectedProduct")))))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 1)))))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::inCart::cart")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::cart")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::cart")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "cart")))) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::inCart1::cart")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::cart")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::cart")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "cart")))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection")))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1")))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::inCart1")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::inCart1::cart")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::inCart1")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::selectedProduct1")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::selectedProduct1::selectedProduct")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::selectedProduct1")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::withAccount1")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::withAccount1::account")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::withAccount1")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2")))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::account")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (source direct))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (source inherited) (from (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::account"))))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (scopes any))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::account")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::account::withAccount1")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::account")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::cart")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (source direct))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (source inherited) (from (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::cart"))))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::cart")) (scopes any feature))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::cart::inCart1")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::cart")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::selectedProduct")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (source direct))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (source inherited) (from (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::selectedProduct"))))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (scopes any))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::selectedProduct")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::selectedProduct::selectedProducts1")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::selectedProduct")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3")))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::account")))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (source direct))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (source inherited) (from (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::account"))))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (scopes any))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::account")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::cart"))) (cyclic true)
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::cart")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::selectedProduct"))) (cyclic true)
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3")))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::selectedProduct")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 2)) (named (kind kerml-feature) (name "account")))))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 2)))))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "cart")))))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 0)))))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "selectedProduct")))))
      (featured-by (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 1)))))
      (type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 15 33) (end 15 45)) (probe (position 15 33))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::inCart::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 16 54) (end 16 61)) (probe (position 16 54))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::selectedProducts::selectedProduct"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 17 42) (end 17 49)) (probe (position 17 42))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1::withAccount::account"))) (kind featureTyping) (ordinal 0) (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 28 24) (end 28 31)) (probe (position 28 24))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::account"))) (kind featureTyping) (ordinal 0) (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 22 20) (end 22 32)) (probe (position 22 20))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 25 31) (end 25 38)) (probe (position 25 31))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2::selectedProduct"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 35 37) (end 35 48)) (probe (position 35 37))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3"))) (kind specialization) (ordinal 0) (authored-target "Links::Link")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 54 24) (end 54 31)) (probe (position 54 24))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::account"))) (kind featureTyping) (ordinal 0) (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 36 12) (end 36 24)) (probe (position 36 12))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 45 23) (end 45 30)) (probe (position 45 23))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3::selectedProduct"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 10 31) (end 10 38)) (probe (position 10 31))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 2)) (named (kind kerml-feature) (name "account"))))) (kind featureTyping) (ordinal 0) (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 8 27) (end 8 39)) (probe (position 8 27))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "cart"))))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 9 38) (end 9 45)) (probe (position 9 38))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "ProductSelection")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "selectedProduct"))))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 65 42) (end 65 58)) (probe (position 65 42))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection"))) (kind specialization) (ordinal 0) (authored-target "ProductSelection")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 71 43) (end 71 60)) (probe (position 71 43))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1"))) (kind specialization) (ordinal 0) (authored-target "ProductSelection1")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection1")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 72 35) (end 72 47)) (probe (position 72 35))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::inCart1::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 73 55) (end 73 62)) (probe (position 73 55))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::selectedProduct1::selectedProduct"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 74 44) (end 74 51)) (probe (position 74 44))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection1::withAccount1::account"))) (kind featureTyping) (ordinal 0) (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 77 43) (end 77 60)) (probe (position 77 43))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2"))) (kind specialization) (ordinal 0) (authored-target "ProductSelection2")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection2")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 84 24) (end 84 31)) (probe (position 84 24))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::account"))) (kind featureTyping) (ordinal 0) (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 78 20) (end 78 32)) (probe (position 78 20))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 81 31) (end 81 38)) (probe (position 81 31))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection2::selectedProduct"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 89 43) (end 89 60)) (probe (position 89 43))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3"))) (kind specialization) (ordinal 0) (authored-target "ProductSelection3")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ProductSelection3")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 106 24) (end 106 31)) (probe (position 106 24))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::account"))) (kind featureTyping) (ordinal 0) (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 90 12) (end 90 24)) (probe (position 90 12))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 90 38) (end 90 42)) (probe (position 90 38))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::cart"))) (kind redefinition) (ordinal 0) (authored-target "cart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::cart")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 98 23) (end 98 30)) (probe (position 98 23))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::selectedProduct"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 98 44) (end 98 59)) (probe (position 98 44))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::selectedProduct"))) (kind redefinition) (ordinal 0) (authored-target "selectedProduct")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::SingleProductSelection3::selectedProduct")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 68 31) (end 68 38)) (probe (position 68 31))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 2)) (named (kind kerml-feature) (name "account"))))) (kind featureTyping) (ordinal 0) (authored-target "Account")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Account")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 66 27) (end 66 39)) (probe (position 66 27))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "cart"))))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::ShoppingCart")))))
    )
  )
  (query (document "memory://snapshot/product_selection_n_ary.md") (range (start 67 38) (end 67 45)) (probe (position 67 38))
    (reference (id (source (node (document "memory://snapshot/product_selection_n_ary.md") (path (named (kind package) (name "ProductSelection_N_ary")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "selectedProduct"))))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_n_ary.md") (qualified-name "ProductSelection_N_ary::Product")))))
    )
  )
)
~~~
