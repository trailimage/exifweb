import { is, ValueType } from '@toba/tools'
import { JsonLD, contextField, typeField, idField, Type } from './types'

const defaultContext = 'http://schema.org'

export interface Image {
   url: string
   width?: number
   height?: number
}

/**
 * Class generates a JSON-LD representation.
 */
export interface LinkData<T extends JsonLD.Thing> {
   jsonLD(): T
}

/**
 * Update Link Data schema with standard fields.
 */
export function standardize<T extends JsonLD.Thing>(
   type: string,
   schema: Partial<T>
): T {
   if (is.defined(schema, 'id')) {
      // rename ID field to standard
      schema[idField] = schema.id
      delete schema['id']
   }
   schema[typeField] = type
   schema[contextField] = defaultContext
   return schema as T
}

/**
 * Basic Link Data for an image URL and optional dimensions.
 */
export function image(img: Image): JsonLD.ImageObject {
   const schema: JsonLD.ImageObject = { url: img.url }
   if (is.value(img.width)) {
      schema.width = img.width
   }
   if (is.value(img.height)) {
      schema.height = img.height
   }
   return standardize<JsonLD.ImageObject>(Type.ImageObject, schema)
}

/**
 * Basic Link data for place with a map URL.
 */
export function place(mapURL: string): JsonLD.Place {
   return standardize<JsonLD.Place>(Type.Place, { hasMap: mapURL })
}

/**
 * Basic Link Data for a web page.
 */
export function webPage(url: string): JsonLD.WebPage {
   return standardize<JsonLD.WebPage>(Type.WebPage, { id: url })
}

/**
 * Basic Link Data for an organization.
 */
export function organization(title: string, logo?: Image): JsonLD.Organization {
   const schema: JsonLD.Organization = { name: title }
   if (is.value<Image>(logo)) {
      schema.logo = image(logo)
   }
   return standardize<JsonLD.Organization>(Type.Organization, schema)
}

export function breadcrumb(
   url: string,
   title: string,
   position: number
): JsonLD.Breadcrumb {
   const schema: JsonLD.Breadcrumb = { item: { id: url, name: title } }
   if (!isNaN(position)) {
      schema.position = position
   }
   return standardize<JsonLD.Breadcrumb>(Type.Breadcrumb, schema)
}

export function discoverAction(url: string): JsonLD.DiscoverAction {
   return standardize<JsonLD.DiscoverAction>(Type.DiscoverAction, {
      target: url
   })
}

/**
 * Remove redundant context specifications.
 */
export function removeContext(linkData: JsonLD.Thing, context?: string) {
   if (is.value(linkData) && typeof linkData == ValueType.Object) {
      if (
         is.defined(linkData, contextField) &&
         linkData[contextField] !== null
      ) {
         if (context !== null && linkData[contextField] == context) {
            // remove redundant value
            delete linkData[contextField]
         } else {
            // switch to new context
            context = linkData[contextField]
         }
      }
      for (const field in linkData) {
         removeContext(linkData[field], context)
      }
   }
}

/**
 * Convert link data to string with nulls and zeroes removed.
 */
export function serialize(linkData: any): string {
   removeContext(linkData)
   return JSON.stringify(linkData, (_key, value) =>
      value === null || value === 0 ? undefined : value
   )
}
