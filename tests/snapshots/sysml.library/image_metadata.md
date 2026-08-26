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
  (document "memory://snapshot/image_metadata.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 8 16) (end 8 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 22) (end 17 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 23) (end 25 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 19) (end 34 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 23) (end 41 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2b83cf119754d6bc9559e41ab3d581375d06e0eb4db5deaf119afeb46265bdf4") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package provides attributive data and metadata to allow a model element to be\n\t * annotated with an image to be used in its graphical rendering or as a marker to\n\t * adorn graphical or textual renderings.\n\t "))))
    (declaration (id (node (document "memory://snapshot/image_metadata.md") (path (named (kind library-package) (name "ImageMetadata")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::String") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon"))) (kind metadata-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * Icon metadata can be used to annotate a model element with an image to be used\n\t\t * to show render the element on a diagram and/or a small image to be used as an\n\t\t * adornment on a graphical or textual rendering. Alternatively, another metadata\n\t\t * definition can be annotated with an Icon to indicate that any model element \n\t\t * annotated by the containing metadata can be rendered according to the Icon.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon::fullImage"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (documentation (doc (text "\n\t\t\t * A full-sized image that can be used to render the annotated element on a\n\t\t\t * graphical view, potentially as an alternative to its standard rendering.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image")))))
    (declaration (id (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon::smallImage"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (documentation (doc (text "\n\t\t\t * A smaller image that can be used as an adornment on the graphical rendering\n\t\t\t * of the annotated element or as a marker in a textual rendering.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Image")))))
    (declaration (id (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * Image provides the data necessary for the physical definition of \n\t\t * a graphical image.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image::content"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (documentation (doc (text "\n\t\t\t * Binary data for the image according to the given MIME type, \n\t\t\t * encoded as given by the encoding.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image::encoding"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (documentation (doc (text "\n\t\t\t * Describes how characters in the content are to be decoded into \n\t\t\t * binary data. At least \"base64\", \"hex\", \"identify\", and \"JSONescape\"\n\t\t\t * shall be supported.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image::location"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (documentation (doc (text "\n\t\t\t * A URI for the location of a resource containing the image content,\n\t\t\t * as an alternative for embedding it in the content attribute.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image::type"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (documentation (doc (text "\n\t\t\t * The MIME type according to which the content should be interpreted.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/image_metadata.md") (path (named (kind library-package) (name "ImageMetadata")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon::fullImage"))) (kind featureTyping) (ordinal 0))
      (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image")))))
    (reference (id (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon::smallImage"))) (kind featureTyping) (ordinal 0))
      (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image")))))
    (reference (id (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image::content"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image::encoding"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image::location"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image::type"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon::fullImage"))) (target (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon::fullImage"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon::smallImage"))) (target (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon::smallImage"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon::fullImage"))) (target (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon::smallImage"))) (target (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image::content"))) (target (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image::encoding"))) (target (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image::location"))) (target (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image::type"))) (target (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon::fullImage")))
      (featured-by (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon")))
      (type (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image")) (provenance authored))
      (effective-type (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image")) (source direct))
      (supertype (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon::smallImage")))
      (featured-by (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon")))
      (type (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image")) (provenance authored))
      (effective-type (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image")) (source direct))
      (supertype (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image")))
      (subtype (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon::fullImage")) (scopes any))
      (subtype (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon::smallImage")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image::content")))
      (featured-by (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image")))
    )
    (declaration (id (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image::encoding")))
      (featured-by (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image")))
    )
    (declaration (id (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image::location")))
      (featured-by (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image")))
    )
    (declaration (id (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image::type")))
      (featured-by (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/image_metadata.md") (range (start 8 16) (end 8 36)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/image_metadata.md") (path (named (kind library-package) (name "ImageMetadata")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/image_metadata.md") (range (start 60 24) (end 60 29)) (probe (position 60 24))
    (reference (id (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon::fullImage"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image")))))
    )
  )
  (query (document "memory://snapshot/image_metadata.md") (range (start 68 25) (end 68 30)) (probe (position 68 25))
    (reference (id (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Icon::smallImage"))) (kind featureTyping) (ordinal 0) (authored-target "Image")
      (outcome (status resolved) (target (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image")))))
    )
  )
  (query (document "memory://snapshot/image_metadata.md") (range (start 17 22) (end 17 28)) (probe (position 17 22))
    (reference (id (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image::content"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/image_metadata.md") (range (start 25 23) (end 25 29)) (probe (position 25 23))
    (reference (id (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image::encoding"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/image_metadata.md") (range (start 41 23) (end 41 29)) (probe (position 41 23))
    (reference (id (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image::location"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/image_metadata.md") (range (start 34 19) (end 34 25)) (probe (position 34 19))
    (reference (id (source (node (document "memory://snapshot/image_metadata.md") (qualified-name "ImageMetadata::Image::type"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
)
~~~
