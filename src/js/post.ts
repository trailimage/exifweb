/**
 * Data attributes on post photos
 * @example
 * `<img src="/whatever.webp" data-src="/medium.webp" data-big="/large.webp"/>`
 */
const enum LargeImage {
   URL = 'big',
   Loaded = 'big_loaded',
   Height = 'big_height',
   Width = 'big_width'
}

const enum MediumImage {
   URL = 'src'
}

interface LazyLoadOptions {
   rootMargin?: string
   root?: Element | null
   threshold?: number | number[]
   /** Milliseconds to delay image load */
   delayLoad?: number
}

/**
 * Enable lazy loading and light box for post images. Depends on post images
 * having data-src, data-big, data-big-width and data-big-height attributes.
 */
window.addEventListener('DOMContentLoaded', () => {
   const htmlTag = document.getElementsByTagName('html')[0]! as HTMLHtmlElement
   /** Post photos */
   const photos = Array.from(
      document.querySelectorAll('figure > img')! as NodeListOf<HTMLImageElement>
   )
   /** Light box container */
   const lb = document.getElementById('light-box')! as HTMLDivElement

   // TODO: activate EXIF

   /**
    * Depends on page having a "light-box" element containing a simgle `img` tag
    *
    * ```
    * <div id="light-box"><img src="[something]"/></div>
    * ```
    */
   class LightBox {
      /** Element re-used as a light box for any clicked post photo */
      private el: HTMLDivElement
      private bigPhoto: HTMLImageElement
      private clickedPhoto: HTMLImageElement
      private size: Size

      constructor(photos: HTMLImageElement[], el: HTMLDivElement) {
         this.el = el
         this.bigPhoto = this.el.getElementsByTagName('img')[0]!

         // clicking an image opens it in a lightbox
         photos.forEach(p => p.addEventListener('click', (e) => this.onPhotoClick(e)))

         // Hide light box when clicked and re-enable page scrolling
         this.on('click', this.hide.bind(this))
      }

      /** Whether big photo is already browser cached */
      private get isLoaded(): boolean {
         return this.data(LargeImage.Loaded) == "true"
      }

      private set isLoaded(value: boolean) {
         this.clickedPhoto.dataset[LargeImage.Loaded] = value ? 'true' : 'false'
      }

      private show() {
         this.el.style.display = 'block'
         htmlTag.style.overflow = 'hidden'
      }
      private hide() {
         this.el.style.display = 'none'
         window.removeEventListener('resize', this.updateSize.bind(this))
         htmlTag.style.overflow = 'auto'
      }

      /** Value of key within clicked photo dataset */
      private data(key: string): string {
         return this.clickedPhoto.dataset[key]!
      }

      private on(eventName: string, handler: (e: Event) => void) {
         this.el.addEventListener(eventName, handler)
      }

      private off(eventName: string, handler: (e: Event) => void) {
         this.el.removeEventListener(eventName, handler)
      }

      private onPhotoClick(event: MouseEvent) {
         event.preventDefault()

         this.clickedPhoto = event.target as HTMLImageElement

         this.size = new Size(
            this.data(LargeImage.Width), this.data(LargeImage.Height)
         )

         if (this.isLoaded) {
            this.bigPhoto.src = this.data(LargeImage.URL)
         } else {
            // assign lower resolution image while the bigger one is loading
            this.bigPhoto.src = this.data(MediumImage.URL)
            this.download().then(() => {
               this.bigPhoto.src = this.data(LargeImage.URL)
               this.isLoaded = true
            })
         }
         this.bigPhoto.style.height = this.size.height.image.toString()
         this.bigPhoto.style.width = this.size.width.image.toString()

         this.updateSize(event)
         this.show()

         // update panning calculations if window resizes
         window.addEventListener('resize', this.updateSize.bind(this))
      }

      /**
       * Create temporary element to download image so the image is fully cached
       * before being assigned to the light box
       */
      private async download() {
         /** Temporary element used to cache image */
         const loader = document.createElement('img')

         return new Promise(resolve => {
            loader.addEventListener('load', resolve)
            loader.src = this.data(LargeImage.URL)
         })
      }

      /** Move the big image by the given number of pixels */
      private move(x: number, y: number) {
         this.bigPhoto.style.transform = `translate(${x}px, ${y}px)`
      }

      private updateSize(event: MouseEvent) {
         let cursor = 'zoom-out'

         this.size.update()

         if (this.size.needsToPan) {
            cursor = 'move'
            this.on('mousemove', this.updateHoverPosition.bind(this))
         } else {
            this.off('mousemove', this.updateHoverPosition.bind(this))
         }
         // set initial desktop position and cursor
         this.updateHoverPosition(event)
         this.bigPhoto.style.cursor = cursor
      }

      /** Update image position within light box */
      private updateHoverPosition(event: MouseEvent) {
         const x = event.clientX
         const y = event.clientY

         if (x !== undefined && y !== undefined) {
            const dx = this.size.width.offset(x)
            const dy = this.size.height.offset(y)
            this.move(dx, dy)
         }
      }

   }

   /**
    * ```
    *  ╔════════╤════════════════╗
    *  ║        │ extra          ║
    *  ║   ╔════╧═══╤════════╗   ║
    *  ║   ║        │ from   ║   ║
    *  ║   ║        ┼ center ║   ║
    *  ║   ║ window          ║   ║
    *  ║   ╚═════════════════╝   ║
    *  ║ image                   ║
    *  ╚═════════════════════════╝
    * ```
    * Represent image width or height dimension compared to the window to
    * calculate panning amount and speed
    */
   class Length {
      /** Image edge length */
      image: number
      /** Window edge length */
      window: number
      /** How much longer is window edge (usually a negative number) */
      extra: number
      /** Ratio of mouse to image movement pixels for panning */
      panRatio: number

      constructor(forImage: string) {
         this.image = parseInt(forImage)
         this.window = 0
         this.extra = 0
         this.panRatio = 0
      }

      /**
       * Update window dimension and calculate how much larger it is than image
       */
      update(forWindow: number) {
         this.window = forWindow
         this.extra = (this.window - this.image) / 2
      }

      /**
       * Calculate ratio for this dimension. Leading number is factor by which
       * to accelerate panning so edge of image is visible before cursor
       * reaches edge of window.
       */
      ratio(): number {
         return 2 * ((this.window - this.image) / this.window)
      }

      /**
       * Get image offset based on mouse position
       * @param m Current mouse position in this dimension
       */
      offset(m: number): number {
         const subtract =
            this.extra > 0 ? 0 : (this.window / 2 - m) * this.panRatio

         return this.extra - subtract
      }

      /**
       * Get image offset necessary to center the image
       */
      center(): number {
         return this.extra / 2
      }
   }

   /**
    * Represent image size.
    */
   class Size {
      width: Length
      height: Length
      /** Whether image needs to pan because it is larger than the window */
      needsToPan: boolean

      constructor(imageWidth: string, imageHeight: string) {
         this.width = new Length(imageWidth)
         this.height = new Length(imageHeight)
      }

      /**
       * Update calculations if window is resized
       */
      update() {
         this.height.update(window.innerHeight)
         this.width.update(window.innerWidth)
         this.needsToPan = this.width.extra < 0 || this.height.extra < 0

         if (this.needsToPan) {
            // pan image using length with biggest ratio
            // or if one dimension needs no panning then use the other dimension
            this.height.panRatio = this.width.panRatio =
               this.width.extra < this.height.extra && this.width.extra < 0
                  ? this.width.ratio()
                  : this.height.ratio()
         }
      }
   }

   /**
    * Based on Lazy Load plugin by Mika Tuupola
    * @see https://appelsiini.net/projects/lazyload
    */
   class LazyLoader {
      images: HTMLImageElement[]
      options: LazyLoadOptions
      observer: IntersectionObserver

      private lazyTimer = 'timerid'

      constructor(images: HTMLImageElement[]) {
         this.options = {
            root: null,
            rootMargin: '0px',
            threshold: 0,
            delayLoad: 300,
         }

         this.images = images

         if (window.IntersectionObserver) {
            this.observe()
         } else {
            // pre-load all image if no observer available
            console.warn('Browser does not support IntersectionObserver')
            this.images.forEach(this.loadImage)
         }
      }

      /** Get timer ID stored as element data */
      private getTimer(el: HTMLImageElement): number {
         const textID = el.dataset[this.lazyTimer]
         return textID === undefined ? 0 : parseInt(textID)
      }

      private clearTimer(el: HTMLImageElement) {
         delete el.dataset[this.lazyTimer]
      }

      /** Store timer ID in element data */
      private setTimer(el: HTMLImageElement, id: number) {
         el.dataset[this.lazyTimer] = id.toString()
      }

      private delayLoad(el: HTMLImageElement) {
         let timerID: number = this.getTimer(el)

         if (timerID == 0) {
            timerID = setTimeout(() => {
               this.observer.unobserve(el)
               this.loadImage(el)
               this.clearTimer(el)
            }, this.options.delayLoad)

            this.setTimer(el, timerID)
         }
      }

      private cancelLoad(el: HTMLImageElement) {
         const timerID: number = this.getTimer(el)
         if (timerID > 0) {
            clearTimeout(timerID)
            this.clearTimer(el)
         }
      }

      private observe() {
         this.observer = new IntersectionObserver(
            (entries) => {
               entries.forEach((e) => {
                  const el = e.target as HTMLImageElement
                  if (e.isIntersecting) {
                     this.delayLoad(el)
                  } else {
                     this.cancelLoad(el)
                  }
               })
            },
            {
               root: this.options.root,
               rootMargin: this.options.rootMargin,
               threshold: this.options.threshold,
            }
         )
         this.images.forEach((el: Element) => this.observer.observe(el))
      }

      private loadImage(el: HTMLImageElement) {
         el.src = el.dataset[MediumImage.URL]!
      }
   }

   new LazyLoader(photos)
   new LightBox(photos, lb)
})
