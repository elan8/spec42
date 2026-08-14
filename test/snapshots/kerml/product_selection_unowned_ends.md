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
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 25 17) (end 25 50))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 28 3) (end 29 2))
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
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::cart"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart")))))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::info"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SelectionInfo")))))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::selectedProduct"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product")))))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SelectionInfo"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection"))) (kind kerml-association) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ProductSelection")))))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1"))) (kind kerml-association) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ProductSelection")))))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::cart"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart")) (redefinition (reference "cart")))))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (path (named (kind package) (name "ProductSelection_UnownedEnds")) (named (kind kerml-association) (name "SingleProductSelection1")) (named (kind kerml-feature) (name "cart")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "selectedProduct::selectedProduct1") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::selectedProduct"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product")) (redefinition (reference "selectedProduct")))))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (path (named (kind package) (name "ProductSelection_UnownedEnds")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 0))))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (path (named (kind package) (name "ProductSelection_UnownedEnds")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "selectedProduct"))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product")))))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection::cart"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::info"))) (kind featureTyping) (ordinal 0))
      (authored-target "SelectionInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SelectionInfo")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection"))) (kind specialization) (ordinal 0))
      (authored-target "ProductSelection")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1"))) (kind specialization) (ordinal 0))
      (authored-target "ProductSelection")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::cart"))) (kind redefinition) (ordinal 0))
      (authored-target "cart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::cart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (path (named (kind package) (name "ProductSelection_UnownedEnds")) (named (kind kerml-association) (name "SingleProductSelection1")) (named (kind kerml-feature) (name "cart")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "selectedProduct::selectedProduct1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::selectedProduct"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::selectedProduct"))) (kind redefinition) (ordinal 0))
      (authored-target "selectedProduct")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::selectedProduct")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (path (named (kind package) (name "ProductSelection_UnownedEnds")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "selectedProduct"))))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::cart"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::info"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SelectionInfo"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::info"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::cart"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::cart"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::cart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::cart"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::selectedProduct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::selectedProduct"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::selectedProduct"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (path (named (kind package) (name "ProductSelection_UnownedEnds")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "selectedProduct"))))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (path (named (kind package) (name "ProductSelection_UnownedEnds")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "selectedProduct"))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection::cart"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection::cart"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::cart"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::Product")))
      (subtype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::selectedProduct")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::selectedProduct")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_unowned_ends.md") (path (named (kind package) (name "ProductSelection_UnownedEnds")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "selectedProduct")))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection")))
      (subtype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::cart")))
      (featured-by (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection")))
      (type (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection::cart")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::info")))
      (featured-by (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection")))
      (type (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SelectionInfo")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SelectionInfo")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SelectionInfo")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::selectedProduct")))
      (featured-by (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection")))
      (type (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::Product")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::Product")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::Product")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SelectionInfo")))
      (subtype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::info")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart")))
      (subtype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::cart")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::cart")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection::cart")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection")))
      (supertype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1")))
      (supertype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::cart"))) (cyclic true)
      (featured-by (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1")))
      (type (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::cart")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (path (named (kind package) (name "ProductSelection_UnownedEnds")) (named (kind kerml-association) (name "SingleProductSelection1")) (named (kind kerml-feature) (name "cart")) (anonymous (kind import) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::cart")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::selectedProduct"))) (cyclic true)
      (featured-by (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1")))
      (type (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::Product")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::Product")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::Product")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::selectedProduct")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (path (named (kind package) (name "ProductSelection_UnownedEnds")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (path (named (kind package) (name "ProductSelection_UnownedEnds")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "selectedProduct")))))
      (featured-by (node (document "memory://snapshot/product_selection_unowned_ends.md") (path (named (kind package) (name "ProductSelection_UnownedEnds")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 0)))))
      (type (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::Product")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::Product")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::Product")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection::cart")))
      (featured-by (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection")))
      (type (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart")) (source direct))
      (effective-type (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart")) (source inherited) (from (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::cart"))))
      (supertype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::cart")) (scopes any feature))
      (supertype (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 13 20) (end 13 32)) (probe (position 13 20))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart")))))
    )
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 11 16) (end 11 29)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::info"))) (kind featureTyping) (ordinal 0) (authored-target "SelectionInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SelectionInfo")))))
    )
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 14 31) (end 14 38)) (probe (position 14 31))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::Product")))))
    )
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 17 33) (end 17 49)) (probe (position 17 33))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection"))) (kind specialization) (ordinal 0) (authored-target "ProductSelection")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection")))))
    )
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 23 34) (end 23 50)) (probe (position 23 34))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1"))) (kind specialization) (ordinal 0) (authored-target "ProductSelection")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ProductSelection")))))
    )
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 24 20) (end 24 32)) (probe (position 24 20))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart")))))
    )
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 24 46) (end 24 50)) (probe (position 24 46))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::cart"))) (kind redefinition) (ordinal 0) (authored-target "cart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::cart")))))
    )
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 25 17) (end 25 50)) (probe (position 25 17))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (path (named (kind package) (name "ProductSelection_UnownedEnds")) (named (kind kerml-association) (name "SingleProductSelection1")) (named (kind kerml-feature) (name "cart")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "selectedProduct::selectedProduct1")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 27 31) (end 27 38)) (probe (position 27 31))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::selectedProduct"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::Product")))))
    )
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 27 52) (end 27 67)) (probe (position 27 52))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::selectedProduct"))) (kind redefinition) (ordinal 0) (authored-target "selectedProduct")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection1::selectedProduct")))))
    )
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 19 38) (end 19 45)) (probe (position 19 38))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (path (named (kind package) (name "ProductSelection_UnownedEnds")) (named (kind kerml-association) (name "SingleProductSelection")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "selectedProduct"))))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::Product")))))
    )
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 18 20) (end 18 32)) (probe (position 18 20))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::SingleProductSelection::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds::ShoppingCart")))))
    )
  )
)
~~~
