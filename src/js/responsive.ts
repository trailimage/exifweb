interface PageFeature {
   /** Whether the Facebook comment API should be loaded */
   facebook: boolean
}

/** Defined in /views/post.hbs */
declare const pageFeatures: PageFeature

/**
 * Only load scripts and data for the current view port and features.
 */
window.addEventListener('DOMContentLoaded', () => {
   /** Whether mobile resources have been loaded */
   let mobileLoaded = false
   /** Whether desktop resources have been loaded */
   let desktopLoaded = false
   let timer = 0
   /** Page width below which mobile rather than desktop resources will be loaded */
   const breakAt = 1024
   const feature: PageFeature = Object.assign({ facebook: false }, pageFeatures)

   window.addEventListener('resize', onResize)

   // always check on first load
   checkResources()

   /**
    * Load different resources if view size crosses break boundary
    */
   function onResize() {
      if (mobileLoaded && desktopLoaded) {
         // no need to check after everything is loaded
         window.removeEventListener('resize', onResize)
      } else {
         if (timer > 0) window.clearTimeout(timer)
         timer = window.setTimeout(checkResources, 500)
      }
   }

   /**
    * Load resources based on current view width
    */
   function checkResources() {
      const width = window.innerWidth
      if (width === undefined || width > breakAt) {
         loadDesktop()
      } else {
         loadMobile()
      }
   }

   /**
    * Find category selectors and attach event handlers
    */
   function prepareMenu(menuTag: HTMLElement) {
      const onSelect = (e: Event) => {
         const url = (e.target as HTMLSelectElement).value
         window.location.assign(url)
      }
      const menus = menuTag.getElementsByTagName('select')! as HTMLCollectionOf<HTMLSelectElement>

      for (let el of Array.from(menus)) {
         el.addEventListener('select', onSelect)
         el.addEventListener('change', onSelect)
      }
   }

   /**
    * Lazy-load mobile resources. Menu is loaded in closed state. Only attach
    * handlers after it's first opened.
    */
   function loadMobile() {
      if (mobileLoaded) return

      const imageStyle = { width: '100%', height: 'auto' }
      const menu = document.getElementById('mobile-menu')!
      const body = document.getElementsByName('body')[0]! as HTMLBodyElement

      loadHTML(menu, '/mobile-menu').then(() => {
         let visible = false
         let prepared = false
         const openButton = document.getElementById('mobile-menu-button')!
         const close = () => {
            menu.style.display = "none"
            visible = false
            body.style.position = "static"
         }
         const prepare = () => {
            visible = true
            if (prepared) return

            const closeButton = menu.getElementsByClassName('close')[0]!
            closeButton.addEventListener('click', close)

            prepareMenu(menu)
         }

         openButton.addEventListener('click', () => {
            if (visible) {
               close()
            } else {
               body.style.position = 'fixed'
               menu.style.display = 'block'
               prepare()
            }

         })
      })

      // make post images fill width
      // TODO: fill width still correct?
      // $('figure, .category.content a.thumb').each(function (this: HTMLElement) {
      //    $(this).css(imageStyle).find('img').css(imageStyle)
      // })

      mobileLoaded = true
   }

   /**
    * Lazy-load desktop resources
    */
   function loadDesktop() {
      if (desktopLoaded) return

      const menu = document.getElementById('category-menu')!

      loadHTML(menu, '/category-menu').then(prepareMenu)

      if (feature.facebook) {
         loadScript(
            'facebook-jssdk',
            '//connect.facebook.net/en_US/all.js#xfbml=1&appId=110860435668134',
            true
         )
      }

      desktopLoaded = true
   }

   function loadHTML(target: HTMLElement, url: string): Promise<HTMLElement> {
      return fetch(url)
         .then(res => res.text())
         .then(html => {
            target.innerHTML = html
            return target
         })
   }

   /**
    * Load a remote JavaScript source
    */
   function loadScript(id: string, url: string, async: boolean = false) {
      let js
      const firstScript = document.getElementsByTagName('script')[0]

      if (!document.getElementById(id)) {
         if (async === undefined) async = false

         js = document.createElement('script')
         js.id = id
         js.src = url
         js.async = async

         const parent = firstScript.parentNode

         if (parent === null) {
            console.error('Failed to load script source')
         } else {
            parent.insertBefore(js, firstScript)
         }
      }
   }
})
