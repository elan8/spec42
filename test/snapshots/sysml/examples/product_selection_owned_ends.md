# META
~~~ini
description=SysML Example (Association): ProductSelection_OwnedEnds
type=file
~~~
# SOURCE
~~~sysml
package ProductSelection_OwnedEnds_SysML {
	
	item def SelectionInfo;
	item def ShoppingCart;
	item def Product;
	
	// User-specified connection defiation definition
	connection def ProductSelection {
		item info: SelectionInfo;
		
		end [0..1] item cart: ShoppingCart[1];
		end [0..*] nonunique item selectedProduct: Product[1];
	}
	
	// Equivalent connection defiation definition with named end items.
	connection def ProductSelection1 {
		item info: SelectionInfo;
		
		end inCart[0..1] item cart: ShoppingCart[1];
		end selectedProducts[0..*] item selectedProduct: Product[1];
	}
	
	connection def SingleProductSelection specializes ProductSelection {
		end [0..1] item cart: ShoppingCart[1];
		end [0..1] item selectedProduct: Product[1];
	}

	connection def SingleProductSelection1 specializes ProductSelection1 {
		end inCart1 [0..1] item cart: ShoppingCart[1];
		end selectedProduct1 [0..1] item selectedProduct: Product[1];
	}
	
	item def OnlineCustomer {
		item info1: SelectionInfo;	
		item myCart: ShoppingCart[1];	
		item products: Product[0..*];
		
		connection ps1 : ProductSelection connect myCart to products {
			:>> info = info1;
		}
		
		connection ps2 : ProductSelection connect [1] myCart to [1] products {
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
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 37 2) (end 39 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 41 2) (end 43 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:7c9673e92202057393274f21f1ea83af5b2885e93f1e41486a28f843024696f1") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::info1"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SelectionInfo")))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::myCart"))) (kind item) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart")))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::products"))) (kind item) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product")))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::Product"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1::inCart"))) (kind connection) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1::info"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SelectionInfo")))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1::selectedProducts"))) (kind connection) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::cart"))) (kind connection) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart")))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::info"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SelectionInfo")))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::nonunique"))) (kind connection) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection"))) (kind connection-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ProductSelection")))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1"))) (kind connection-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ProductSelection1")))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1::inCart1"))) (kind connection) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1::selectedProduct1"))) (kind connection) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::cart"))) (kind connection) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart")))))
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::selectedProduct"))) (kind connection) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::info1"))) (kind featureTyping) (ordinal 0))
      (authored-target "SelectionInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::myCart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::products"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1::info"))) (kind featureTyping) (ordinal 0))
      (authored-target "SelectionInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::info"))) (kind featureTyping) (ordinal 0))
      (authored-target "SelectionInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection"))) (kind specialization) (ordinal 0))
      (authored-target "ProductSelection")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1"))) (kind specialization) (ordinal 0))
      (authored-target "ProductSelection1")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::Product")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::info1"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::info1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::myCart"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::myCart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::products"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::products"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1::info"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1::info"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::cart"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::info"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::info"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::cart"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::cart"))) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::cart"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::info1")))
      (featured-by (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer")))
      (type (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::myCart")))
      (featured-by (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer")))
      (type (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::products")))
      (featured-by (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer")))
      (type (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::Product")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::Product")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::Product")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::Product")))
      (subtype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::products")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::selectedProduct")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection")))
      (subtype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1")))
      (subtype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1::inCart")))
      (featured-by (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1::info")))
      (featured-by (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1")))
      (type (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1::selectedProducts")))
      (featured-by (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::cart")))
      (featured-by (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection")))
      (type (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::cart")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::info")))
      (featured-by (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection")))
      (type (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::nonunique")))
      (featured-by (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo")))
      (subtype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::info1")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1::info")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::info")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart")))
      (subtype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::myCart")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::cart")) (scopes any))
      (subtype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::cart")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection")))
      (supertype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1")))
      (supertype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1::inCart1")))
      (featured-by (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1::selectedProduct1")))
      (featured-by (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1")))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::cart")))
      (featured-by (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection")))
      (type (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart")) (source direct))
      (effective-type (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart")) (source inherited) (from (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::cart"))))
      (supertype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::cart")) (scopes any feature))
      (supertype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::selectedProduct")))
      (featured-by (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection")))
      (type (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::Product")) (provenance authored))
      (effective-type (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::Product")) (source direct))
      (supertype (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::Product")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 33 14) (end 33 27)) (probe (position 33 14))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::info1"))) (kind featureTyping) (ordinal 0) (authored-target "SelectionInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo")))))
    )
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 34 15) (end 34 27)) (probe (position 34 15))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::myCart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart")))))
    )
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 35 17) (end 35 24)) (probe (position 35 17))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer::products"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::Product")))))
    )
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 16 13) (end 16 26)) (probe (position 16 13))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1::info"))) (kind featureTyping) (ordinal 0) (authored-target "SelectionInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo")))))
    )
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 10 24) (end 10 36)) (probe (position 10 24))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart")))))
    )
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 8 13) (end 8 26)) (probe (position 8 13))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::info"))) (kind featureTyping) (ordinal 0) (authored-target "SelectionInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo")))))
    )
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 22 51) (end 22 67)) (probe (position 22 51))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection"))) (kind specialization) (ordinal 0) (authored-target "ProductSelection")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection")))))
    )
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 27 52) (end 27 69)) (probe (position 27 52))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1"))) (kind specialization) (ordinal 0) (authored-target "ProductSelection1")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1")))))
    )
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 23 24) (end 23 36)) (probe (position 23 24))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart")))))
    )
  )
  (query (document "memory://snapshot/product_selection_owned_ends.md") (range (start 24 35) (end 24 42)) (probe (position 24 35))
    (reference (id (source (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_owned_ends.md") (qualified-name "ProductSelection_OwnedEnds_SysML::Product")))))
    )
  )
)
~~~
