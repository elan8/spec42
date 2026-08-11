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
  (document "product_selection_unowned_ends.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 41) (end 13 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 47) (end 14 68))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e74dcc4b31fc5b5224952863aba076ef433fd10c8c73a047e67fe0f315423026") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML"))) (kind "package") (name "ProductSelection_UnownedEnds_SysML") (declared-name "ProductSelection_UnownedEnds_SysML"))
    (element (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::OnlineCustomer"))) (kind "item def") (name "OnlineCustomer") (declared-name "OnlineCustomer") (parent (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::Product"))) (kind "item def") (name "Product") (declared-name "Product") (parent (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection"))) (kind "connection def") (name "ProductSelection") (declared-name "ProductSelection") (parent (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::cart"))) (kind "interface end") (name "cart") (declared-name "cart") (parent (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection"))) (authored (relationships (typing (reference "ShoppingCart")) (cross-subsetting (reference "selectedProduct.inCart")))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::selectedProduct"))) (kind "interface end") (name "selectedProduct") (declared-name "selectedProduct") (parent (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection"))) (authored (relationships (typing (reference "Product")) (cross-subsetting (reference "cart.selectedProducts")))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SelectionInfo"))) (kind "item def") (name "SelectionInfo") (declared-name "SelectionInfo") (parent (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart"))) (kind "item def") (name "ShoppingCart") (declared-name "ShoppingCart") (parent (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection"))) (kind "connection def") (name "SingleProductSelection") (declared-name "SingleProductSelection") (parent (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ProductSelection")))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::cart"))) (kind "interface end") (name "cart") (declared-name "cart") (parent (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection"))) (authored (relationships (typing (reference "ShoppingCart")))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::selectedProduct"))) (kind "interface end") (name "selectedProduct") (declared-name "selectedProduct") (parent (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection"))) (authored (relationships (typing (reference "Product")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::cart"))) (kind featureTyping) (ordinal 0)) (authored-target "ShoppingCart") (outcome (status resolved) (target (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart")))))
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::cart"))) (kind crossSubsetting) (ordinal 0)) (authored-target "selectedProduct.inCart") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0)) (authored-target "Product") (outcome (status resolved) (target (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::Product")))))
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::selectedProduct"))) (kind crossSubsetting) (ordinal 0)) (authored-target "cart.selectedProducts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection"))) (kind specialization) (ordinal 0)) (authored-target "ProductSelection") (outcome (status resolved) (target (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection")))))
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::cart"))) (kind featureTyping) (ordinal 0)) (authored-target "ShoppingCart") (outcome (status resolved) (target (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart")))))
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0)) (authored-target "Product") (outcome (status resolved) (target (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::Product")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::cart"))) (target (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::selectedProduct"))) (target (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::Product"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection"))) (target (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::cart"))) (target (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::selectedProduct"))) (target (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::Product"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 17 42) (end 17 58)) (probe (position 17 42))
      (reference
        (source (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::SingleProductSelection"))
        (kind specialization) (ordinal 0) (authored-target "ProductSelection")
        (range (start 17 42) (end 17 58))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection") (range (start 10 1) (end 10 206)))
        )
      )
    )
    (query (range (start 14 47) (end 14 68)) (probe (position 14 47))
      (reference
        (source (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::selectedProduct"))
        (kind crossSubsetting) (ordinal 0) (authored-target "cart.selectedProducts")
        (range (start 14 47) (end 14 68))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 41) (end 13 63)) (probe (position 13 41))
      (reference
        (source (document "d0") (qualified-name "ProductSelection_UnownedEnds_SysML::ProductSelection::cart"))
        (kind crossSubsetting) (ordinal 0) (authored-target "selectedProduct.inCart")
        (range (start 13 41) (end 13 63))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
