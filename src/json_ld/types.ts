export const contextField = '@context'
export const typeField = '@type'
export const idField = '@id'

export declare namespace JsonLD {
   /**
    * An action performed by a direct agent and indirect participants upon a
    * direct object. Optionally happens at a location with the help of an
    * inanimate instrument. The execution of the action may produce a result.
    * Specific action sub-type documentation specifies the exact expectation of
    * each argument/role.
    *
    * @see http://schema.org/Action
    */
   export interface Action extends Thing {
      actionStatus: ActionStatusType
      agent: Person | Organization
      participant: Person | Organization
      endTime: Date
      error: Thing
      instrument: Thing
      location: Place | PostalAddress | string
      object: Thing
      result: Thing
      startTime: Date
      target: EntryPoint | string
   }

   /**
    * The status of an Action.
    * @see http://schema.org/ActionStatusType
    */
   export interface ActionStatusType extends Thing {}

   /**
    * A geographical region, typically under the jurisdiction of a particular government.
    * @see http://schema.org/AdministrativeArea
    */
   export interface AdministrativeArea extends Place {}

   /**
    * The average rating based on multiple ratings or reviews.
    * @see http://schema.org/AggregateRating
    */
   export interface AggregateRating extends Rating {
      itemReviews: Thing
      ratingCount: number
      reviewCount: number
   }

   /**
    * An article, such as a news article or piece of investigative report.
    * Newspapers and magazines have articles of many different types and this is
    * intended to cover them all.
    *
    * @see http://schema.org/Article
    */
   export interface Article extends CreativeWork {
      articleBody?: string
      articleSection?: string
      pageStart?: string | number
      pageEnd?: string | number
      pagination?: string
      wordCount?: number
   }

   /**
    * Intended audience for an item, i.e. the group for whom the item was
    * created.
    * @see http://schema.org/Audience
    */
   export interface Audience extends Thing {
      audienceType: string
      geographicArea: AdministrativeArea
   }

   /**
    * A blog.
    * @see http://schema.org/Blog
    */
   export interface Blog extends Thing {
      blogPost: BlogPosting[]
   }

   /**
    * A blog post.
    * @see http://schema.org/BlogPosting
    */
   export interface BlogPosting extends SocialMediaPosting {}

   /**
    * A brand is a name used by an organization or business person for labeling
    * a product, product group, or similar.
    * @see http://schema.org/Brand
    */
   export interface Brand extends Thing {
      aggregateRating?: AggregateRating
      review?: Review
      logo?: string
   }

   /**
    * A set of links that can help a user understand and navigate a website
    * hierarchy.
    *
    * @see http://schema.org/breadcrumb
    */
   export interface Breadcrumb extends Thing {}

   /**
    * A `BreadcrumbList` is an `ItemList` consisting of a chain of linked Web
    * pages, typically described using at least their URL and their name, and
    * typically ending with the current page.
    *
    * The `position` property is used to reconstruct the order of the items in a
    * BreadcrumbList The convention is that a breadcrumb list has an
    * `itemListOrder` of `ItemListOrderAscending` (lower values listed first),
    * and that the first items in this list correspond to the "top" or beginning
    * of the breadcrumb trail, e.g. with a site or section homepage. The
    * specific values of `position` are not assigned meaning for a
    * `BreadcrumbList`, but they should be integers, e.g. beginning with `1` for
    * the first item in the list.
    *
    * @see http://schema.org/BreadcrumbList
    */
   export interface BreadcrumbList extends ListItem<Breadcrumb> {}

   /**
    * A business entity type is a conceptual entity representing the legal form,
    * the size, the main line of business, the position in the value chain, or
    * any combination thereof, of an organization or business person.
    *
    * Commonly used values:
    * - http://purl.org/goodrelations/v1#Business
    * - http://purl.org/goodrelations/v1#Enduser
    * - http://purl.org/goodrelations/v1#PublicInstitution
    * - http://purl.org/goodrelations/v1#Reseller
    *
    * @see http://schema.org/BusinessEntityType
    */
   export interface BusinessEntityType extends Thing {}

   /**
    * The business function specifies the type of activity or access (i.e., the
    * bundle of rights) offered by the organization or business person through
    * the offer. Typical are sell, rental or lease, maintenance or repair,
    * manufacture / produce, recycle / dispose, engineering / construction, or
    * installation. Proprietary specifications of access rights are also
    * instances of this class.
    *
    * Commonly used values:
    * - http://purl.org/goodrelations/v1#ConstructionInstallation
    * - http://purl.org/goodrelations/v1#Dispose
    * - http://purl.org/goodrelations/v1#LeaseOut
    * - http://purl.org/goodrelations/v1#Maintain
    * - http://purl.org/goodrelations/v1#ProvideService
    * - http://purl.org/goodrelations/v1#Repair
    * - http://purl.org/goodrelations/v1#Sell
    * - http://purl.org/goodrelations/v1#Buy
    *
    * @see http://schema.org/BusinessFunction
    */
   export interface BusinessFunction extends Thing {}

   /**
    * A comment on an item - for example, a comment on a blog post. The
    * comment's content is expressed via the text property, and its topic via
    * about, properties shared with all `CreativeWork`s.
    *
    * @see http://schema.org/Comment
    */
   export interface Comment extends CreativeWork {
      downvoteCount: number
      upvoteCount: number
   }

   /**
    * A country.
    * @see http://schema.org/Country
    */
   export interface Country extends Place {}

   /**
    * The most generic kind of creative work, including books, movies,
    * photographs, software programs, etc.
    * @see http://schema.org/CreativeWork
    */
   export interface CreativeWork extends Thing {
      author?: Person | Organization
      creator?: Person | Organization
      provider?: Person | Organization
      producer?: Person | Organization
      sourceOrganization?: Organization
      editor?: Person
      associatedArticle?: NewsArticle
      requiresSubscription?: boolean
      contentSize?: string
      contentUrl?: URL | string
      encodingFormat?: string
      bitrate?: string
      duration?: Duration
      height?: Distance | QuantitativeValue | number
      width?: Distance | QuantitativeValue | number
      productionCompany?: Organization
      regionsAllowed?: Place
      copyrightHolder?: Person | Organization
      copyrightYear?: number
      audience?: Audience
      encoding?: MediaObject
      hasPart?: CreativeWork
      isPartOf?: CreativeWork
      headling?: string
      keywords?: string
      locationCreated?: Place
      review?: Review
      datePublished?: DateTime
      text?: string
      version?: number
      mainEntity?: Thing
      thumbnailUrl?: string
   }

   /**
    * A contact point — for example, a Customer Complaints department.
    * @see http://schema.org/ContactPoint
    */
   export interface ContactPoint extends Thing {
      areaServed: AdministrativeArea | GeoShape | Place | string
      availableLanguage: Language | string
      contactOption: ContactPointOption
      contactType: string
      email: string
      faxNumber: string
      hoursAvailable: OpeningHoursSpecification
      productSupported: Product | string
      telephone: string
   }

   /**
    * Enumerated options related to a ContactPoint.
    * @see http://schema.org/ContactPointOption
    */
   export interface ContactPointOption extends Thing {}

   /**
    * The geographic coordinates of a place or event.
    * @see http://schema.org/GeoCoordinates
    */
   export interface Coordinates extends Thing {
      /** Physical address of the item. */
      address: string | PostalAddress
      /**
       * The country. For example, `USA`. You can also provide the two-letter
       * ISO 3166-1 alpha-2 country code.
       */
      addressCountry: string | Country
      elevation: string | number
      latitude: string | number
      longitude: string | number
      postalCode: string
   }

   /**
    * A combination of date and time of day in the form
    * `[-]CCYY-MM-DDThh:mm:ss[Z|(+|-)hh:mm]`
    *
    * @see Chapter 5.4 of ISO 8601
    * @see http://schema.org/DateTime
    * @see https://en.wikipedia.org/wiki/ISO_8601
    */
   export interface DateTime extends Thing {}

   /**
    * The day of the week, e.g. used to specify to which day the opening hours
    * of an OpeningHoursSpecification refer.
    * @see http://schema.org/DayOfWeek
    */
   export interface DayOfWeek extends Thing {}

   /**
    * A delivery method is a standardized procedure for transferring the product
    * or service to the destination of fulfillment chosen by the customer.
    * Delivery methods are characterized by the means of transportation used,
    * and by the organization or group that is the contracting party for the
    * sending organization or person.
    *
    * Commonly used values:
    * - http://purl.org/goodrelations/v1#DeliveryModeDirectDownload
    * - http://purl.org/goodrelations/v1#DeliveryModeFreight
    * - http://purl.org/goodrelations/v1#DeliveryModeMail
    * - http://purl.org/goodrelations/v1#DeliveryModeOwnFleet
    * - http://purl.org/goodrelations/v1#DeliveryModePickUp
    * - http://purl.org/goodrelations/v1#DHL
    * - http://purl.org/goodrelations/v1#FederalExpress
    * - http://purl.org/goodrelations/v1#UPS
    *
    * @see http://schema.org/DeliveryMethod
    */
   export interface DeliveryMethod extends Thing {
      method: {
         directDownload: 'http://purl.org/goodrelations/v1#DeliveryModeDirectDownload'
         freight: 'http://purl.org/goodrelations/v1#DeliveryModeFreight '
         mail: 'http://purl.org/goodrelations/v1#DeliveryModeMail'
         ownFleet: 'http://purl.org/goodrelations/v1#DeliveryModeOwnFleet'
         pickUp: 'http://purl.org/goodrelations/v1#DeliveryModePickUp'
         DHL: 'http://purl.org/goodrelations/v1#DHL'
         federalExpress: 'http://purl.org/goodrelations/v1#FederalExpress'
         UPS: 'http://purl.org/goodrelations/v1#UPS'
      }
   }

   /**
    * A demand entity represents the public, not necessarily binding, not
    * necessarily exclusive, announcement by an organization or person to seek a
    * certain type of goods or services. For describing demand using this type,
    * the very same properties used for `Offer` apply.
    *
    * @see http://schema.org/Demand
    */
   export interface Demand extends Thing {
      /** The payment method(s) accepted by seller for this offer. */
      acceptedPaymentMethod: LoanOrCredit | PaymentMethod
      /**
       * The amount of time that is required between accepting the offer and the
       * actual usage of the resource or service.
       */
      advancedBookingRequirement: QuantitativeValue
      areaServed: AdministrativeArea | GeoShape | Place | string
      /**
       * The availability of this item—for example In stock, Out of stock
       * Pre-order, etc.
       */
      availability: ItemAvailability
      availabilityEnds: Date
      availabilityStarts: Date
      availableAtOrFrom: Place
      availableDeliveryMethod: DeliveryMethod
      /**
       * The business function (e.g. sell, lease, repair, dispose) of the offer
       * or component of a bundle (`TypeAndQuantityNode`). The default is
       * `http://purl.org/goodrelations/v1#Sell`.
       */
      businessFunction: BusinessFunction
      /**
       * The typical delay between the receipt of the order and the goods either
       * leaving the warehouse or being prepared for pickup, in case the
       * delivery method is on site pickup.
       */
      deliveryLeadTime: QuantitativeValue
      eligibleCustomerType: BusinessEntityType
      eligibleDuration: QuantitativeValue
      eligibleQuantity: QuantitativeValue
      /**
       * The ISO 3166-1 (ISO 3166-1 alpha-2) or ISO 3166-2 code, the place, or
       * the `GeoShape` for the geo-political region(s) for which the offer or
       * delivery charge specification is valid.
       */
      eligibleRegion: GeoShape | Place | string
      eligibleTransactionVolume: PriceSpecification
      /**
       * The `GTIN-12` code of the product, or the product to which the offer
       * refers. The `GTIN-12` is the 12-digit GS1 Identification Key composed
       * of a U.P.C. Company Prefix, Item Reference, and Check Digit used to
       * identify trade items.
       */
      gtin12: string
      /**
       * The `GTIN-13` code of the product, or the product to which the offer
       * refers. This is equivalent to 13-digit ISBN codes and EAN UCC-13.
       * Former 12-digit UPC codes can be converted into a GTIN-13 code by
       * simply adding a preceeding zero.
       */
      gtin13: string
      gtin14: string
      /**
       * The `GTIN-8` code of the product, or the product to which the offer
       * refers. This code is also known as `EAN/UCC-8` or 8-digit `EAN`.
       */
      gtin8: string
      includesObject: TypeAndQuantityNode
      ineligibleRegion: GeoShape | Place | Text
      inventoryLevel: QuantitativeValue
      itemCondition: OfferItemCondition
      itemOffered: Product | Service
      /**
       * The Manufacturer Part Number (MPN) of the product, or the product to
       * which the offer refers.
       */
      mpn: string
      priceSpecification: PriceSpecification
      seller: Organization | Person
      /**
       * The serial number or any alphanumeric identifier of a particular
       * product. When attached to an offer, it is a shortcut for the serial
       * number of the product included in the offer.
       */
      serialNumber: string
      /**
       * The Stock Keeping Unit (SKU), i.e. a merchant-specific identifier for a
       * product or service, or the product to which the offer refers.
       */
      sku: string
      validFrom: Date
      validThrough: Date
      warranty: WarrantyPromise
   }

   /**
    * A posting to a discussion forum.
    * @see http://schema.org/DiscussionForumPosting
    */
   export interface DiscussionForumPosting extends SocialMediaPosting {}

   /**
    * Properties that take Distances as values are of the form
    * `<Number> <Length unit of measure>`.
    *
    * @example 7 ft
    * @see http://schema.org/Distance
    */
   export interface Distance extends Thing {}

   /**
    * ISO 8601 duration.
    * @see http://schema.org/Duration
    * @see https://en.wikipedia.org/wiki/ISO_8601
    */
   export interface Duration extends Thing {}

   /**
    * An educational organization.
    * @see http://schema.org/EducationalOrganization
    */
   export interface EducationalOrganization extends Organization {
      alumni: Person[]
   }

   /**
    * An entry point, within some Web-based protocol.
    * @see http://schema.org/EntryPoint
    */
   export interface EntryPoint extends Thing {
      /** An application that can complete the request. */
      actionApplication: SoftwareApplication
      /**
       * The high level platform(s) where the `Action` can be performed for the
       * given URL. To specify a specific application or operating system
       * instance, use `actionApplication`.
       */
      actionPlatform: string | URL
      contentType: string
      encodingType: string
      /**
       * An HTTP method that specifies the appropriate HTTP method for a request
       * to an HTTP EntryPoint. Values are capitalized strings as used in HTTP.
       */
      httpMethod: string
      /**
       * An url template (RFC6570) that will be used to construct the target of
       * the execution of the action.
       */
      urlTemplate: string
   }

   /**
    * Lists or enumerations—for example, a list of cuisines or music genres,
    * etc.
    * @see http://schema.org/Enumeration
    */
   export interface Enumeration extends Thing {}

   /**
    * An event happening at a certain time and location, such as a concert,
    * lecture, or festival. Ticketing information may be added via the `offers`
    * property. Repeated events may be structured as separate `Event` objects.
    *
    * @see http://schema.org/Event
    */
   export interface Event extends Thing {
      actor: Person
      aggregateRating: AggregateRating
      attendee: Person[] | Organization[]
      composer: Person | Organization
      contributor: Person[] | Organization[]
      director: Person
      doorTime: Date
      duration: Duration
      endDate: Date
      eventStatus: EventStatusType
      funder: Organization | Person
      inLanguage: Language | string
      isAccessibleForFree: boolean
      location: Place | PostalAddress | string
      offers: Offer[]
      organizer: Person | Organization
      performer: Person | Organization
      previousStartDate: Date
      recordedIn: CreativeWork
      review: Review
      sponsor: Person | Organization
      startDate: Date
      subEvent: Event
      superEvent: Event
      translator: Person | Organization
      typicalAgeRange: string
      workFeatured: CreativeWork
      workPerformed: CreativeWork
   }

   /**
    * @see http://schema.org/EventStatusType
    */
   export interface EventStatusType extends Thing {}

   /**
    * @see http://schema.org/FinancialProduct
    */
   export interface FinancialProduct extends Service {
      annualPercentageRate: number | QuantitativeValue
      feesAndCommissionsSpecification: string | URL
      interestRate: number | QuantitativeValue
   }

   /**
    * @see http://schema.org/GenderType
    */
   export interface GenderType extends Thing {}

   /**
    * @see http://schema.org/GeoCoordinates
    */
   export interface GeoCoordinates extends Thing {
      address: PostalAddress | string
      addressCountry: Country | string
      elevation: number | string
      latitude: number | string
      longitude: number | string
      postalCode: string
   }

   export interface GeoShape extends Thing {
      address: PostalAddress | string
      addressCountry: Country | string
      box: string
      circle: string
      elevation: string | number
      line: string
      polygon: string
      postalCode: string
   }

   /**
    * @see http://schema.org/ImageObject
    */
   export interface ImageObject extends MediaObject {
      caption?: string
      exifData?: PropertyValue | string
      representativeOfPage?: boolean
      thumbnail?: ImageObject
   }

   /**
    * @see http://schema.org/ItemAvailability
    */
   export interface ItemAvailability extends Thing {}

   /**
    * @see http://schema.org/ItemList
    */
   export interface ItemList<T extends Thing> extends Thing {
      itemListElement: ListItem<T>[] | string[] | Thing[]
      itemListOrder: ItemListOrderType | string
      numberofItems: number
   }

   /**
    * @see http://schema.org/ItemListOrderType
    */
   export interface ItemListOrderType extends Thing {}

   /**
    * @see http://schema.org/Language
    */
   export interface Language extends Thing {}

   /**
    * @see http://schema.org/ListItem
    */
   export interface ListItem<T extends Thing> extends Thing {
      item: T
      nextItem: ListItem<T>
      position: number | string
      previousItem: ListItem<T>
   }

   /**
    * @see http://schema.org/LoanOrCredit
    */
   export interface LoanOrCredit extends FinancialProduct {
      amount: MonetaryAmount | number
      loanTerm: QuantitativeValue
      requiredCollateral: Thing | string
   }

   /**
    * @see http://schema.org/LocationFeatureSpecification
    */
   export interface LocationFeatureSpecification extends PropertyValue {
      hoursAvailable: OpeningHoursSpecification
      validFrom: Date
      validThrough: Date
   }

   export interface MediaObject extends CreativeWork {
      embedUrl?: URL | string
      encodesCreativeWork?: CreativeWork
      expires?: Date
      playerType?: string
      productionCompany?: Organization
      regionsAllowed?: Place
      requiresSubscription?: boolean
      uploadDate?: Date
   }

   /**
    * @see http://schema.org/MonetaryAmount
    */
   export interface MonetaryAmount extends Thing {
      currency: string
      maxValue: number
      minValue: number
      validFrom: Date
      validThrough: Date
      value: boolean | number | StructuredValue | string
   }

   /**
    * @see http://schema.org/MusicAlbum
    */
   export interface MusicAlbum extends CreativeWork {
      albumProductionType: MusicAlbumProductionType
      albumRelease: MusicRelease
      albumReleaseType: MusicAlbumReleaseType
      byArtist: MusicGroup
   }

   /**
    * @see http://schema.org/MusicAlbumProductionType
    */
   export interface MusicAlbumProductionType extends Thing {}

   /**
    * @see http://schema.org/MusicAlbumReleaseType
    */
   export interface MusicAlbumReleaseType extends Thing {}

   /**
    * @see http://schema.org/MusicComposition
    */
   export interface MusicComposition extends CreativeWork {
      composer: Person | Organization
      firstPerformance: Event
      includedComposition: MusicComposition
      iswcCode: string
      lyricist: Person
      lyrics: CreativeWork
      musicArrangement: MusicComposition
      musicCompositionForm: string
      musicalKey: string
      recordedAs: MusicRecording
   }

   /**
    * @see http://schema.org/MusicGroup
    */
   export interface MusicGroup extends Organization {
      album: MusicAlbum
      genre: string | URL
      track: ItemList<MusicRecording> | MusicRecording
   }

   /**
    * @see http://schema.org/MusicPlaylist
    */
   export interface MusicPlaylist extends CreativeWork {
      numTracks: number
      track: ItemList<MusicRecording> | MusicRecording
   }

   /**
    * @see http://schema.org/MusicRecording
    */
   export interface MusicRecording extends CreativeWork {
      byArtist: MusicGroup
      duration: Duration
      inAlbum: MusicAlbum
      inPlaylist: MusicPlaylist
      isrcCode: string
      recordingOf: MusicComposition
   }

   /**
    * @see http://schema.org/MusicRelease
    */
   export interface MusicRelease extends MusicPlaylist {
      catalogNumber: string
      creditedTo: Person | Organization
      duration: Duration
      musicReleaseFormat: MusicReleaseFormatType
      recordLabel: Organization
      realseOf: MusicAlbum
   }

   /**
    * @see http://schema.org/MusicReleaseFormatType
    */
   export interface MusicReleaseFormatType extends Thing {}

   export interface NewsArticle extends Article {
      dateline: string
      printColumn: string
      printEdition: string
      printPage: string
      printSelection: string
   }

   export interface Offer extends Thing {
      acceptingPaymentMethod: LoanOrCredit | PaymentMethod
      addOn: Offer
      advancedBookingRequirement: QuantitativeValue
      aggregateRating: AggregateRating
      areaServiced: AdministrativeArea | GeoShape | Place | string
      availability: ItemAvailability
      availabilityEnds: Date
      availabilityStarts: Date
      availableAtOrFrom: Place
      availableDeliveryMethod: DeliveryMethod
      businessFunction: BusinessFunction
      category: Thing | string
      deliveryLeadTime: QuantitativeValue
      eligibleCustomerType: BusinessEntityType
      eligibleDuration: QuantitativeValue
      eligibleQuantity: QuantitativeValue
      eligibleRegion: GeoShape | Place | string
      eligibleTransactionVolume: PriceSpecification
      gtin12: string
      gtin13: string
      gtin14: string
      gtin8: string
      includesObject: TypeAndQuantityNode
      ineligibleRegion: GeoShape | Place | string
      inventoryLevel: QuantitativeValue
      itemCondition: OfferItemCondition
      itemOffered: Product | Service
      mpn: string
      offeredBy: Organization | Person
      price: number | string
      priceCurrency: string
      priceSpecification: PriceSpecification
      priceValidUntil: Date
      review: Review
      seller: Organization | Person
      serialNumber: string
      sku: string
      validFrom: Date
      validThrough: Date
      warranty: WarrantyPromise
   }

   /**
    * @see http://schema.org/OfferCatalog
    */
   export interface OfferCatalog extends ItemList<Offer> {}

   /**
    * @see http://schema.org/OfferItemCondition
    */
   export interface OfferItemCondition extends Thing {}

   /**
    * @see http://schema.org/OpeningHoursSpecification
    */
   export interface OpeningHoursSpecification extends Thing {
      closes: Date
      dayOfWeek: DayOfWeek
      opens: Date
      validFrom: Date
      validThrough: Date
   }

   /**
    * @see http://schema.org/Organization
    */
   export interface Organization extends Thing {
      address?: PostalAddress | string
      aggregateRating?: AggregateRating
      alumni?: Person[]
      areaServiced?: AdministrativeArea | GeoShape | Place | string
      award?: string
      brand?: Brand | Organization
      contactPoint?: ContactPoint
      department?: Organization
      dissolutionDate?: Date
      duns?: string
      email?: string
      employee?: Person | Person[]
      event?: Event
      faxNumber?: string
      founder?: Person | Person[]
      foundingDate?: Date
      foundingLocation?: Place
      funder?: Organization | Person
      globalLocationNumber?: string
      hasOfferCatalog?: OfferCatalog
      hasPOS?: Place
      isicV4?: string
      legalName?: string
      leiCode?: string
      location?: Place | PostalAddress | string
      logo?: ImageObject | URL
      makesOffer?: Offer
      member?: Organization | Person
      memberOf?: Organization | Person
      naics?: string
      numberOfEmployees?: QuantitativeValue
      owns?: OwnershipInfo | Product
      parentOrganization?: Organization
      review?: Review
      seeks?: Demand
      sponsor?: Organization | Person
      subOrganization?: Organization
      taxID?: string
      telephone?: string
      vatID?: string
   }

   /**
    * @see http://schema.org/OwnershipInfo
    */
   export interface OwnershipInfo extends Thing {
      acquiredFrom: Organization | Person
      ownedFrom: Date
      ownedThrough: Date
      typeOfGood: Product | Service
   }

   /**
    * @see http://schema.org/PaymentMethod
    */
   export interface PaymentMethod extends Thing {}

   /**
    * @see http://schema.org/Person
    */
   export interface Person extends Thing {
      additionalName: string
      addres: PostalAddress | string
      affiliation: Organization
      alumniOf: EducationalOrganization[] | Organization[]
      award: string
      birthDate: string
      birthPlace: Place
      brand: Brand | Organization
      children: Person[]
      colleague: Person | URL
      contactPoint: ContactPoint
      deathDate: Date
      deathPlace: Place
      duns: string
      email: string
      familyName: string
      faxNumber: string
      follows: Person[]
      funder: Organization | Person
      gender: GenderType | string
      givenName: string
      globalLocationNumber: string
      hasOfferCatalog: OfferCatalog
      hasPOS: Place
      height: Distance | QuantitativeValue
      homeLocation: ContactPoint | Place
      honorificPrefix: string
      honorificSuffix: string
      isicV4: string
      jobTitle: string
      knows: Person[]
      makesOffer: Offer
      memberOf: Organization[] | ProgramMembership[]
      naics: string
      nationality: Country
      netWorth: MonetaryAmount | PriceSpecification
      owns: OwnershipInfo[] | Product[]
      parent: Person[]
      performerIn: Event
      relatedTo: Person[]
      seeks: Demand[]
      sibling: Person[]
      sponder: Organization | Person
      spouse: Person
      taxID: string
      telephone: string
      vatID: string
      weight: QuantitativeValue
      workLocation: ContactPoint | Place
      worksFor: Organization
   }

   /**
    * @see http://schema.org/Photograph
    */
   export interface Photograph extends CreativeWork {}

   export interface Place extends Thing {
      additionalProperty: PropertyValue
      address: PostalAddress | string
      aggregateRating: AggregateRating
      amenityFeature: LocationFeatureSpecification
      branchCode: string
      containedInPlace: Place
      containsPlace: Place
      event: Event
      faxNumber: string
      geo: GeoCoordinates | GeoShape
      globalLocationNumber: string
      hasMap: Map<string, URL> | URL | string
      isicV4: string
      logo: ImageObject | URL
      openingHoursSpecification: OpeningHoursSpecification
      photo: ImageObject | Photograph
      review: Review
      smokingAllowed: boolean
      specialOpeningHoursSpecification: OpeningHoursSpecification
      telephone: string
   }

   export interface PostalAddress extends ContactPoint {
      addressCountry: Country | string
      addressLocality: string
      addressRegion: string
      postOfficeBoxNumber: string
      postalCode: string
      streetAddress: string
   }

   /**
    * @see http://schema.org/PriceSpecification
    */
   export interface PriceSpecification extends Thing {
      eligibleQuantity: QuantitativeValue
      eligibleTransactionVolume: PriceSpecification
      maxPrice: number
      minPrice: number
      price: number | string
      priceCurrency: string
      validFrom: Date
      validThrough: Date
      valueAddedTaxIncluded: boolean
   }

   /**
    * @see http://schema.org/ProgramMembership
    */
   export interface ProgramMembership extends Thing {
      hostingOrganization: Organization
      member: Organization | Person
      membershipNumber: string
      programName: string
   }

   export interface Product extends Thing {
      additionalProperty: PropertyValue
      aggregateRating: AggregateRating
      audience: Audience
      award: string
      brand: Brand | Organization
      category: string | Thing
      color: string
      depth: Distance | QuantitativeValue
      gtin12: string
      gtin13: string
      gtin14: string
      gtin8: string
      height: Distance | QuantitativeValue
      isAccessoryOrSparePart: Product
      isConsumableFor: Product
      isRelatedTo: Product | Service
      itemCondition: OfferItemCondition
      logo: ImageObject | URL
      manufacturer: Organization
      model: ProductModel | string
      mpn: string
      offers: Offer
      productID: string
      productionDate: Date
      purchaseDate: Date
      releaseDate: Date
      review: Review
      sku: string
      weight: QuantitativeValue
      width: Distance | QuantitativeValue
   }

   /**
    * @see http://schema.org/ProductModel
    */
   export interface ProductModel extends Product {
      isVariantOf: ProductModel
      predecessorOf: ProductModel
      successorOf: ProductModel
   }

   export interface PropertyValue extends Thing {
      maxValue: number
      minValue: number
      propertyID: string | URL
      unitCode: string | URL
      unitText: string
      value: boolean | number | StructuredValue | string
      valueReference:
         | Enumeration
         | PropertyValue
         | QualitativeValue
         | QuantitativeValue
         | StructuredValue
   }

   export interface QualitativeValue extends Thing {
      additionalProperty: PropertyValue
      equal: QualitativeValue
      greater: QualitativeValue
      greaterOrEqual: QualitativeValue
      lesser: QualitativeValue
      lesserOrEqual: QualitativeValue
      nonEqual: QualitativeValue
      valueReference:
         | Enumeration
         | PropertyValue
         | QualitativeValue
         | QuantitativeValue
         | StructuredValue
   }

   export interface QuantitativeValue extends Thing {
      additionalProperty: PropertyValue
      maxValue: number
      minValue: number
      unitCode: string | URL
      unitText: string
      value: boolean | number | StructuredValue | string
      valueReference:
         | Enumeration
         | PropertyValue
         | QualitativeValue
         | QuantitativeValue
         | StructuredValue
   }

   /**
    * @see http://schema.org/Rating
    */
   export interface Rating extends Thing {
      author: Organization | Person
      bestRating: string | number
      ratingValue: string | number
      worstRating: string | number
   }

   /**
    * @see http://schema.org/Review
    */
   export interface Review extends CreativeWork {
      itemReviewed: Thing
      reviewBody: string
      reviewRating: Rating
   }

   /**
    * @see http://schema.org/SearchAction
    */
   export interface SearchAction extends Action {
      query: string
   }

   /**
    * @see http://schema.org/DiscoverAction
    */
   export interface DiscoverAction extends Action {}

   /**
    * @see http://schema.org/Service
    */
   export interface Service extends Thing {
      aggregateRating: AggregateRating
      areaServed: AdministrativeArea | GeoShape | Place | string
      audience: Audience
      availableChannel: ServiceChannel
      award: string
      brand: Brand | Organization
      category: string | Thing
      hasOfferCatalog: OfferCatalog
      hoursAvailable: OpeningHoursSpecification
      isRelatedTo: Product | Service
      isSimilarTo: Product | Service
      logo: ImageObject | URL
      offers: Offer[]
      provider: Organization | Person
      providerMobility: string
      review: Review
      serviceOutput: Thing
      serviceType: string
   }

   /**
    * @see http://schema.org/ServiceChannel
    */
   export interface ServiceChannel extends Thing {
      availableLanguage: Language
      processingTime: Duration
      providesService: Service
      serviceLocation: Place
      servicePhone: ContactPoint
      servicePostalAddress: PostalAddress
      serviceSmsNumber: ContactPoint
      serviceURL: URL
   }

   /**
    * @see http://schema.org/SocialMediaPosting
    */
   export interface SocialMediaPosting extends Article {
      sharedContent?: CreativeWork
   }

   /**
    * @see http://schema.org/SoftwareApplication
    */
   export interface SoftwareApplication extends Thing {
      applicationCategory: string
      applicationSuite: string
      downloadUrl: string
      operatingSystem: string
      softwareVersion: string
   }

   /**
    * @see http://schema.org/Specialty
    */
   export interface Specialty extends Thing {}

   /**
    * @see http://schema.org/StructuredValue
    */
   export interface StructuredValue extends Thing {}

   export interface Thing {
      [key: string]: any
      [idField]?: string
      [contextField]?: string
      [typeField]?: string
      name?: string
      description?: string
      image?: ImageObject | string
      alternateName?: string
      additionalType?: URL
      potentialAction?: Action
      sameAs?: URL
      mainEntityOfPage?: CreativeWork | URL
      url?: URL | string
   }

   /**
    * @see http://schema.org/TypeAndQuantityNode
    */
   export interface TypeAndQuantityNode extends Thing {
      amountOfThisGood: number
      businessFunction: BusinessFunction
      typeOfGood: Product | Service
      unitCode: string | URL
      unitText: string
   }

   export interface URL extends Thing {}

   /**
    * @see http://schema.org/VideoObject
    */
   export interface VideoObject extends MediaObject {
      actor: Person
      caption: string
      director: Person
      musicBy: MusicGroup
      thumbnail: ImageObject
      transcript: string
      videoFrameSize: string
      videoQuality: string
   }

   /**
    * @see http://schema.org/WarrantyPromise
    */
   export interface WarrantyPromise extends Thing {
      durationOfWarranty: QuantitativeValue
      warrantyScope: WarrantyScope
   }

   /**
    * @see http://schema.org/WarrantyScope
    */
   export interface WarrantyScope extends Thing {}

   /*
    * @see http://schema.org/WebPage
    */
   export interface WebPage extends CreativeWork {
      breadcrumb?: BreadcrumbList | Breadcrumb[]
      lastReviewed?: Date
      mainContentOfPage?: WebPageElement
      primaryImageOfPage?: ImageObject
      relatedLink?: URL[]
      reviewedBy?: Person | Organization
      significantLink?: URL
      specialty?: Specialty
   }

   /**
    * @see http://schema.org/WebPageElement
    */
   export interface WebPageElement extends CreativeWork {}

   /**
    * @see http://schema.org/WebSite
    */
   export interface WebSite extends CreativeWork {}
}

export const enum Type {
   Blog = 'Blog',
   BlogPosting = 'BlogPosting',
   Breadcrumb = 'Breadcrumb',
   BreadcrumbList = 'BreadcrumbList',
   DiscoverAction = 'DiscoverAction',
   ImageObject = 'ImageObject',
   Organization = 'Organization',
   Person = 'Person',
   Place = 'Place',
   SearchAction = 'SearchAction',
   VideoObject = 'VideoObject',
   WebPage = 'WebPage',
}
