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
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 11 2) (end 12 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 36 37) (end 36 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 39 38) (end 39 44))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 40 3) (end 41 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 41 17) (end 41 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 43 44) (end 43 50))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 44 3) (end 45 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 45 17) (end 45 29))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 70 3) (end 71 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 71 17) (end 71 50))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 74 3) (end 75 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 75 17) (end 75 30))
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
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1::inCart"))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1::inCart::cart"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1::info"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SelectionInfo"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1::selectedProducts"))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1::selectedProducts::selectedProduct"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2::cart"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2::cart::inCart"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers member) (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2::info"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SelectionInfo"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2::selectedProduct"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2::selectedProduct::selectedProducts"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers member) (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Links::BinaryLink"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3::cart"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart")) (redefinition (reference "source"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "ProductSelection3") (name "cart") (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "selectedProduct::selectedProducts") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3::info"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SelectionInfo"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3::selectedProduct"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product")) (redefinition (reference "target"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "ProductSelection3") (name "selectedProduct") (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "cart::inCart") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "ProductSelection") (anonymous (kind kerml-end) (ordinal 0)))))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "ProductSelection") (anonymous (kind kerml-end) (ordinal 0)) (name "cart"))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection::info"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SelectionInfo"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SelectionInfo"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ProductSelection"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection1"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ProductSelection1"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection1::inCart1"))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection1::inCart1::cart"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection1::selectedProduct1"))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection1::selectedProduct1::selectedProduct"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection2"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ProductSelection2"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection2::cart"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection2::cart::inCart1"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers member) (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection2::selectedProduct"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection2::selectedProduct::selectedProduct1"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers member) (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ProductSelection3"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::cart"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart")) (redefinition (reference "cart"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "SingleProductSelection3") (name "cart") (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "selectedProduct::selectedProduct1") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::selectedProduct"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product")) (redefinition (reference "selectedProduct"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "SingleProductSelection3") (name "selectedProduct") (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "cart::inCart1") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "SingleProductSelection") (anonymous (kind kerml-end) (ordinal 0)))))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "SingleProductSelection") (anonymous (kind kerml-end) (ordinal 1)))))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "SingleProductSelection") (anonymous (kind kerml-end) (ordinal 0)) (name "cart"))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart"))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "SingleProductSelection") (anonymous (kind kerml-end) (ordinal 1)) (name "selectedProduct"))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1::inCart::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1::info"))) (kind featureTyping) (ordinal 0))
      (authored-target "SelectionInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SelectionInfo")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1::selectedProducts::selectedProduct"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2::info"))) (kind featureTyping) (ordinal 0))
      (authored-target "SelectionInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SelectionInfo")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2::selectedProduct"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3"))) (kind specialization) (ordinal 0))
      (authored-target "Links::BinaryLink")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3::cart"))) (kind redefinition) (ordinal 0))
      (authored-target "source")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "ProductSelection3") (name "cart") (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0))
      (authored-target "selectedProduct::selectedProducts")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3::info"))) (kind featureTyping) (ordinal 0))
      (authored-target "SelectionInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SelectionInfo")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3::selectedProduct"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3::selectedProduct"))) (kind redefinition) (ordinal 0))
      (authored-target "target")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "ProductSelection3") (name "selectedProduct") (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0))
      (authored-target "cart::inCart")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "ProductSelection") (anonymous (kind kerml-end) (ordinal 0)) (name "cart"))))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection::info"))) (kind featureTyping) (ordinal 0))
      (authored-target "SelectionInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SelectionInfo")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection"))) (kind specialization) (ordinal 0))
      (authored-target "ProductSelection")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection1"))) (kind specialization) (ordinal 0))
      (authored-target "ProductSelection1")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection1::inCart1::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection1::selectedProduct1::selectedProduct"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection2"))) (kind specialization) (ordinal 0))
      (authored-target "ProductSelection2")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection2::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection2::selectedProduct"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3"))) (kind specialization) (ordinal 0))
      (authored-target "ProductSelection3")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::cart"))) (kind redefinition) (ordinal 0))
      (authored-target "cart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::cart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "SingleProductSelection3") (name "cart") (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0))
      (authored-target "selectedProduct::selectedProduct1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::selectedProduct"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::selectedProduct"))) (kind redefinition) (ordinal 0))
      (authored-target "selectedProduct")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::selectedProduct")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "SingleProductSelection3") (name "selectedProduct") (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0))
      (authored-target "cart::inCart1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "SingleProductSelection") (anonymous (kind kerml-end) (ordinal 0)) (name "cart"))))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "SingleProductSelection") (anonymous (kind kerml-end) (ordinal 1)) (name "selectedProduct"))))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1::inCart::cart"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1::inCart::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1::info"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SelectionInfo"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1::info"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1::selectedProducts::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1::selectedProducts::selectedProduct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2::cart"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2::info"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SelectionInfo"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2::info"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2::selectedProduct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3::cart"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3::info"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SelectionInfo"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3::info"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3::selectedProduct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "ProductSelection") (anonymous (kind kerml-end) (ordinal 0)) (name "cart"))))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "ProductSelection") (anonymous (kind kerml-end) (ordinal 0)) (name "cart"))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection::info"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SelectionInfo"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection::info"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection1"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection1"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection1::inCart1::cart"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection1::inCart1::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection1::selectedProduct1::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection1::selectedProduct1::selectedProduct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection2"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection2"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection2::cart"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection2::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection2::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection2::selectedProduct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::cart"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::cart"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::cart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::cart"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::selectedProduct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::selectedProduct"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::selectedProduct"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "SingleProductSelection") (anonymous (kind kerml-end) (ordinal 0)) (name "cart"))))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "SingleProductSelection") (anonymous (kind kerml-end) (ordinal 0)) (name "cart"))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "SingleProductSelection") (anonymous (kind kerml-end) (ordinal 1)) (name "selectedProduct"))))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "SingleProductSelection") (anonymous (kind kerml-end) (ordinal 1)) (name "selectedProduct"))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection2::cart"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2::cart"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection2::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2::selectedProduct"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 18 33) (end 18 45)) (probe (position 18 33))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1::inCart::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 16 16) (end 16 29)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1::info"))) (kind featureTyping) (ordinal 0) (authored-target "SelectionInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SelectionInfo")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 19 54) (end 19 61)) (probe (position 19 54))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1::selectedProducts::selectedProduct"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 26 20) (end 26 32)) (probe (position 26 20))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 24 16) (end 24 29)) (probe (position 24 16))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2::info"))) (kind featureTyping) (ordinal 0) (authored-target "SelectionInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SelectionInfo")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 29 31) (end 29 38)) (probe (position 29 31))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2::selectedProduct"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 36 37) (end 36 54)) (probe (position 36 37))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3"))) (kind specialization) (ordinal 0) (authored-target "Links::BinaryLink")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 39 12) (end 39 24)) (probe (position 39 12))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 39 38) (end 39 44)) (probe (position 39 38))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3::cart"))) (kind redefinition) (ordinal 0) (authored-target "source")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 41 17) (end 41 50)) (probe (position 41 17))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "ProductSelection3") (name "cart") (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0) (authored-target "selectedProduct::selectedProducts")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 37 16) (end 37 29)) (probe (position 37 16))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3::info"))) (kind featureTyping) (ordinal 0) (authored-target "SelectionInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SelectionInfo")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 43 23) (end 43 30)) (probe (position 43 23))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3::selectedProduct"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 43 44) (end 43 50)) (probe (position 43 44))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3::selectedProduct"))) (kind redefinition) (ordinal 0) (authored-target "target")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 45 17) (end 45 29)) (probe (position 45 17))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "ProductSelection3") (name "selectedProduct") (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0) (authored-target "cart::inCart")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 10 27) (end 10 39)) (probe (position 10 27))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "ProductSelection") (anonymous (kind kerml-end) (ordinal 0)) (name "cart"))))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 8 16) (end 8 29)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection::info"))) (kind featureTyping) (ordinal 0) (authored-target "SelectionInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SelectionInfo")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 49 42) (end 49 58)) (probe (position 49 42))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection"))) (kind specialization) (ordinal 0) (authored-target "ProductSelection")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 54 43) (end 54 60)) (probe (position 54 43))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection1"))) (kind specialization) (ordinal 0) (authored-target "ProductSelection1")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection1")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 55 35) (end 55 47)) (probe (position 55 35))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection1::inCart1::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 56 55) (end 56 62)) (probe (position 56 55))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection1::selectedProduct1::selectedProduct"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 59 43) (end 59 60)) (probe (position 59 43))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection2"))) (kind specialization) (ordinal 0) (authored-target "ProductSelection2")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection2")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 60 20) (end 60 32)) (probe (position 60 20))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection2::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 63 31) (end 63 38)) (probe (position 63 31))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection2::selectedProduct"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 68 43) (end 68 60)) (probe (position 68 43))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3"))) (kind specialization) (ordinal 0) (authored-target "ProductSelection3")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ProductSelection3")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 69 12) (end 69 24)) (probe (position 69 12))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 69 38) (end 69 42)) (probe (position 69 38))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::cart"))) (kind redefinition) (ordinal 0) (authored-target "cart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::cart")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 71 17) (end 71 50)) (probe (position 71 17))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "SingleProductSelection3") (name "cart") (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0) (authored-target "selectedProduct::selectedProduct1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 73 23) (end 73 30)) (probe (position 73 23))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::selectedProduct"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 73 44) (end 73 59)) (probe (position 73 44))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::selectedProduct"))) (kind redefinition) (ordinal 0) (authored-target "selectedProduct")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::SingleProductSelection3::selectedProduct")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 75 17) (end 75 30)) (probe (position 75 17))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "SingleProductSelection3") (name "selectedProduct") (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0) (authored-target "cart::inCart1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 50 27) (end 50 39)) (probe (position 50 27))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "SingleProductSelection") (anonymous (kind kerml-end) (ordinal 0)) (name "cart"))))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::ShoppingCart")))))
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 51 38) (end 51 45)) (probe (position 51 38))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (path (name "ProductSelection_OwnedEnds") (name "SingleProductSelection") (anonymous (kind kerml-end) (ordinal 1)) (name "selectedProduct"))))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds::Product")))))
  )
)
~~~
