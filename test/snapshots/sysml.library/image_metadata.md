# META
~~~ini
description=Standard Library: Domain Libraries/Metadata/ImageMetadata
type=file
~~~
# SOURCE
~~~sysml
standard library package ImageMetadata {
	doc
	/*
	 * This package provides attributive data and metadata to allow a model element to be
	 * annotated with an image to be used in its graphical rendering or as a marker to
	 * adorn graphical or textual renderings.
	 */
	 
	private import ScalarValues::String;
	
	attribute def Image {
		doc
		/*
		 * Image provides the data necessary for the physical definition of 
		 * a graphical image.
		 */
		 
		attribute content : String[0..1] {
			doc
			/*
			 * Binary data for the image according to the given MIME type, 
			 * encoded as given by the encoding.
			 */
		}
		
		attribute encoding : String[0..1] {
			doc
			/*
			 * Describes how characters in the content are to be decoded into 
			 * binary data. At least "base64", "hex", "identify", and "JSONescape"
			 * shall be supported.
			 */
		}
		
		attribute type : String[0..1] {
			doc
			/*
			 * The MIME type according to which the content should be interpreted.
			 */
		}
		
		attribute location : String[0..1] {
			doc
			/*
			 * A URI for the location of a resource containing the image content,
			 * as an alternative for embedding it in the content attribute.
			 */
		}
	}
	
	metadata def Icon {
		doc
		/*
		 * Icon metadata can be used to annotate a model element with an image to be used
		 * to show render the element on a diagram and/or a small image to be used as an
		 * adornment on a graphical or textual rendering. Alternatively, another metadata
		 * definition can be annotated with an Icon to indicate that any model element 
		 * annotated by the containing metadata can be rendered according to the Icon.
		 */
		 
		attribute fullImage : Image[0..1] {
			doc
			/*
			 * A full-sized image that can be used to render the annotated element on a
			 * graphical view, potentially as an alternative to its standard rendering.
			 */
		}
		
		attribute smallImage : Image[0..1] {
			doc
			/*
			 * A smaller image that can be used as an adornment on the graphical rendering
			 * of the annotated element or as a marker in a textual rendering.
			 */
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "image_metadata.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 36))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "18e4e734c7d7ba1946daab05e6a58be06e9ca96f3a023410fb3bc824171fc676") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ImageMetadata"))) (kind "package") (name "ImageMetadata") (declared-name "ImageMetadata"))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Icon"))) (kind "metadata def") (name "Icon") (declared-name "Icon") (parent (node (document "d0") (qualified-name "ImageMetadata"))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Icon::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ImageMetadata::Icon"))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Icon::fullImage"))) (kind "attribute") (name "fullImage") (declared-name "fullImage") (parent (node (document "d0") (qualified-name "ImageMetadata::Icon"))) (authored (membership (kind Feature)) (relationships (typing (reference "Image")))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Icon::fullImage::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ImageMetadata::Icon::fullImage"))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Icon::smallImage"))) (kind "attribute") (name "smallImage") (declared-name "smallImage") (parent (node (document "d0") (qualified-name "ImageMetadata::Icon"))) (authored (membership (kind Feature)) (relationships (typing (reference "Image")))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Icon::smallImage::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ImageMetadata::Icon::smallImage"))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Image"))) (kind "attribute def") (name "Image") (declared-name "Image") (parent (node (document "d0") (qualified-name "ImageMetadata"))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Image::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ImageMetadata::Image"))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Image::content"))) (kind "attribute") (name "content") (declared-name "content") (parent (node (document "d0") (qualified-name "ImageMetadata::Image"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Image::content::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ImageMetadata::Image::content"))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Image::encoding"))) (kind "attribute") (name "encoding") (declared-name "encoding") (parent (node (document "d0") (qualified-name "ImageMetadata::Image"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Image::encoding::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ImageMetadata::Image::encoding"))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Image::location"))) (kind "attribute") (name "location") (declared-name "location") (parent (node (document "d0") (qualified-name "ImageMetadata::Image"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Image::location::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ImageMetadata::Image::location"))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Image::type"))) (kind "attribute") (name "type") (declared-name "type") (parent (node (document "d0") (qualified-name "ImageMetadata::Image"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::Image::type::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ImageMetadata::Image::type"))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::String"))) (kind "import") (name "String") (declared-name "String") (parent (node (document "d0") (qualified-name "ImageMetadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ImageMetadata::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ImageMetadata"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ImageMetadata::Icon::fullImage"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (outcome (status resolved) (target (node (document "d0") (qualified-name "ImageMetadata::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "ImageMetadata::Icon::smallImage"))) (kind featureTyping) (ordinal 0)) (authored-target "Image") (outcome (status resolved) (target (node (document "d0") (qualified-name "ImageMetadata::Image")))))
    (reference (id (source (node (document "d0") (qualified-name "ImageMetadata::Image::content"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "ImageMetadata::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ImageMetadata::Image::encoding"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "ImageMetadata::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ImageMetadata::Image::location"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "ImageMetadata::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ImageMetadata::Image::type"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "ImageMetadata::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ImageMetadata::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ImageMetadata::Icon::fullImage"))) (target (node (document "d0") (qualified-name "ImageMetadata::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ImageMetadata::Icon::fullImage"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ImageMetadata::Icon::smallImage"))) (target (node (document "d0") (qualified-name "ImageMetadata::Image"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ImageMetadata::Icon::smallImage"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ImageMetadata::Image::content"))) (target (node (document "d0") (qualified-name "ImageMetadata::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ImageMetadata::Image::content"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ImageMetadata::Image::encoding"))) (target (node (document "d0") (qualified-name "ImageMetadata::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ImageMetadata::Image::encoding"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ImageMetadata::Image::location"))) (target (node (document "d0") (qualified-name "ImageMetadata::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ImageMetadata::Image::location"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ImageMetadata::Image::type"))) (target (node (document "d0") (qualified-name "ImageMetadata::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ImageMetadata::Image::type"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 8 16) (end 8 36)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "ImageMetadata::String"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
        (range (start 8 16) (end 8 36))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
