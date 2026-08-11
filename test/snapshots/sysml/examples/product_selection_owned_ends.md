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
  (document "product_selection_owned_ends.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 2) (end 11 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 2) (end 18 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 2) (end 19 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 2) (end 28 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 2) (end 29 63))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "cbe2d8ab8d6aba37ddd1398fa302ceb2ccf3e70da11b77c44a82fef92e8d39d7") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML"))) (kind "package") (name "ProductSelection_OwnedEnds_SysML") (declared-name "ProductSelection_OwnedEnds_SysML") (range (start (line 0) (character 0)) (end (line 0) (character 1260))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::OnlineCustomer"))) (kind "item def") (name "OnlineCustomer") (declared-name "OnlineCustomer") (range (start (line 32) (character 1)) (end (line 32) (character 318))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::Product"))) (kind "item def") (name "Product") (declared-name "Product") (range (start (line 4) (character 1)) (end (line 4) (character 18))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection"))) (kind "connection def") (name "ProductSelection") (declared-name "ProductSelection") (range (start (line 7) (character 1)) (end (line 7) (character 166))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1"))) (kind "connection def") (name "ProductSelection1") (declared-name "ProductSelection1") (range (start (line 15) (character 1)) (end (line 15) (character 179))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1::inCart"))) (kind "interface end") (name "inCart") (declared-name "inCart") (range (start (line 18) (character 2)) (end (line 18) (character 46))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1::selectedProducts"))) (kind "interface end") (name "selectedProducts") (declared-name "selectedProducts") (range (start (line 19) (character 2)) (end (line 19) (character 62))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::cart"))) (kind "interface end") (name "cart") (declared-name "cart") (range (start (line 10) (character 2)) (end (line 10) (character 40))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection"))) (authored (relationships (typing (reference "ShoppingCart") (range none)))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::nonunique"))) (kind "interface end") (name "nonunique") (declared-name "nonunique") (range (start (line 11) (character 2)) (end (line 11) (character 56))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SelectionInfo"))) (kind "item def") (name "SelectionInfo") (declared-name "SelectionInfo") (range (start (line 2) (character 1)) (end (line 2) (character 24))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart"))) (kind "item def") (name "ShoppingCart") (declared-name "ShoppingCart") (range (start (line 3) (character 1)) (end (line 3) (character 23))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML"))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection"))) (kind "connection def") (name "SingleProductSelection") (declared-name "SingleProductSelection") (range (start (line 22) (character 1)) (end (line 22) (character 160))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ProductSelection") (range (start (line 22) (character 51)) (end (line 22) (character 67)))))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1"))) (kind "connection def") (name "SingleProductSelection1") (declared-name "SingleProductSelection1") (range (start (line 27) (character 1)) (end (line 27) (character 187))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ProductSelection1") (range (start (line 27) (character 52)) (end (line 27) (character 69)))))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1::inCart1"))) (kind "interface end") (name "inCart1") (declared-name "inCart1") (range (start (line 28) (character 2)) (end (line 28) (character 48))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1::selectedProduct1"))) (kind "interface end") (name "selectedProduct1") (declared-name "selectedProduct1") (range (start (line 29) (character 2)) (end (line 29) (character 63))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::cart"))) (kind "interface end") (name "cart") (declared-name "cart") (range (start (line 23) (character 2)) (end (line 23) (character 40))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection"))) (authored (relationships (typing (reference "ShoppingCart") (range none)))))
    (element (id (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::selectedProduct"))) (kind "interface end") (name "selectedProduct") (declared-name "selectedProduct") (range (start (line 24) (character 2)) (end (line 24) (character 46))) (parent (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection"))) (authored (relationships (typing (reference "Product") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1::inCart"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1::selectedProducts"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::cart"))) (kind featureTyping) (ordinal 0)) (authored-target "ShoppingCart") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart")))))
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::nonunique"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection"))) (kind specialization) (ordinal 0)) (authored-target "ProductSelection") (range (start (line 22) (character 51)) (end (line 22) (character 67))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection")))))
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1"))) (kind specialization) (ordinal 0)) (authored-target "ProductSelection1") (range (start (line 27) (character 52)) (end (line 27) (character 69))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1")))))
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1::inCart1"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1::selectedProduct1"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::cart"))) (kind featureTyping) (ordinal 0)) (authored-target "ShoppingCart") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart")))))
    (reference (id (source (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0)) (authored-target "Product") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::Product")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::cart"))) (target (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection"))) (target (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1"))) (target (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ProductSelection1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection1"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::cart"))) (target (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::ShoppingCart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::cart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::selectedProduct"))) (target (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::Product"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ProductSelection_OwnedEnds_SysML::SingleProductSelection::selectedProduct"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
