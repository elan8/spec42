# META
~~~ini
description=SysML Example (Association): ProductSelection_UnownedEnds
type=file
~~~
# SOURCE
~~~sysml
package ProductSelection_UnownedEnds_SysML {
	
	item def SelectionInfo;
	item def ShoppingCart {
		item selectedProducts : Product[0..*];
	}
	item def Product {
		item inCart: ShoppingCart[0..1];
	}
	
	connection def ProductSelection {
		item info: SelectionInfo[1];
		
		end item cart: ShoppingCart[1] crosses selectedProduct.inCart;
		end item selectedProduct: Product[1] crosses cart.selectedProducts;
	}
	
	connection def SingleProductSelection :> ProductSelection {
		end item cart: ShoppingCart[1];
		end [0..1] item selectedProduct: Product[1];
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
  (document "memory://snapshot/product_selection_unowned_ends.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 27 2) (end 29 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 31 2) (end 33 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:f2a4ce723da5119c047b6cc9820b6a1164487b0922762c1cf12974c0063f3197") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::OnlineCustomer"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::OnlineCustomer::info1"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SelectionInfo"))))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::OnlineCustomer::myCart"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart"))))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::OnlineCustomer::products"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product"))))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::Product"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::Product::inCart"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart"))))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::cart"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart"))))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::info"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SelectionInfo"))))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::selectedProduct"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product"))))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SelectionInfo"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart::selectedProducts"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product"))))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection"))) (kind connection-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ProductSelection"))))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::cart"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShoppingCart"))))
    (declaration (id (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::selectedProduct"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Product"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::OnlineCustomer::info1"))) (kind featureTyping) (ordinal 0))
      (authored-target "SelectionInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SelectionInfo")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::OnlineCustomer::myCart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::OnlineCustomer::products"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::Product::inCart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::info"))) (kind featureTyping) (ordinal 0))
      (authored-target "SelectionInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SelectionInfo")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart::selectedProducts"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::Product")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection"))) (kind specialization) (ordinal 0))
      (authored-target "ProductSelection")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::cart"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart")))))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0))
      (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::Product")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::OnlineCustomer::info1"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SelectionInfo"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::OnlineCustomer::info1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::OnlineCustomer::myCart"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::OnlineCustomer::myCart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::OnlineCustomer::products"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::OnlineCustomer::products"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::Product::inCart"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::Product::inCart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::cart"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::info"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SelectionInfo"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::info"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart::selectedProducts"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart::selectedProducts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::cart"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::Product"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::cart"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::cart"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::selectedProduct"))) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::selectedProduct"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 23 14) (end 23 27)) (probe (position 23 14))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::OnlineCustomer::info1"))) (kind featureTyping) (ordinal 0) (authored-target "SelectionInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SelectionInfo")))))
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 24 15) (end 24 27)) (probe (position 24 15))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::OnlineCustomer::myCart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart")))))
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 25 17) (end 25 24)) (probe (position 25 17))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::OnlineCustomer::products"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::Product")))))
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 7 15) (end 7 27)) (probe (position 7 15))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::Product::inCart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart")))))
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 13 17) (end 13 29)) (probe (position 13 17))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart")))))
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 11 13) (end 11 26)) (probe (position 11 13))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::info"))) (kind featureTyping) (ordinal 0) (authored-target "SelectionInfo")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SelectionInfo")))))
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 14 28) (end 14 35)) (probe (position 14 28))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::Product")))))
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 4 26) (end 4 33)) (probe (position 4 26))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart::selectedProducts"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::Product")))))
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 17 42) (end 17 58)) (probe (position 17 42))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection"))) (kind specialization) (ordinal 0) (authored-target "ProductSelection")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection")))))
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 18 17) (end 18 29)) (probe (position 18 17))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::cart"))) (kind featureTyping) (ordinal 0) (authored-target "ShoppingCart")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart")))))
  )
  (query (document "memory://snapshot/product_selection_unowned_ends.md") (range (start 19 35) (end 19 42)) (probe (position 19 35))
    (reference (id (source (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0) (authored-target "Product")
      (outcome (status resolved) (target (node (document "memory://snapshot/product_selection_unowned_ends.md") (qualified-name "ProductSelection_UnownedEnds_SysML::Product")))))
  )
)
~~~
