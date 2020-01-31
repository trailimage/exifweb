import { is, mapSet, findInSet } from '@toba/tools';
import { JsonLD, LinkData } from '@toba/json-ld';
import { forCategory } from './json-ld';
import { Post } from './index';

/**
 * Post category.
 */
export class Category implements LinkData<JsonLD.Blog | JsonLD.WebPage> {
   title: string;
   /**
    * Slug style key that represents path to category.
    * @example parent/child
    */
   key: string;
   subcategories: Set<Category> = new Set();
   posts: Set<Post> = new Set();

   constructor(key: string, title: string) {
      this.key = key;
      this.title = title;
   }

   //unload(keys:string|string[]):void;

   /**
    * Category with matching key or title.
    */
   getSubcategory(keyOrTitle: string): Category | undefined {
      return findInSet(
         this.subcategories,
         c => c.title === keyOrTitle || c.key === keyOrTitle
      );
   }

   /**
    * Whether subcategory is present with given key or title.
    */
   has(keyOrTitle: string): boolean {
      return this.getSubcategory(keyOrTitle) !== undefined;
   }

   /**
    * Add subcategory and update its key to include parent.
    */
   add(subcat: Category) {
      if (is.value(subcat)) {
         const oldKey = subcat.key;

         subcat.key = this.key + '/' + subcat.key;
         this.subcategories.add(subcat);

         // update posts that reference the category by its old key
         for (const p of subcat.posts) {
            p.categories.delete(oldKey);
            p.categories.set(subcat.key, subcat.title);
         }
      }
   }

   /**
    * Remove post from category and subcategories (primarily for testing).
    */
   removePost(post: Post): this {
      this.posts.delete(post);
      this.subcategories.forEach(s => {
         s.removePost(post);
      });
      return this;
   }

   /**
    * Ensure photos and information are loaded for all posts.
    */
   ensureLoaded(): Promise<any> {
      return Promise.all(
         mapSet(this.posts, p => p.getInfo().then(p => p.getPhotos()))
      );
   }

   /**
    * Whether category is a child as opposed to root category.
    */
   get isChild() {
      return this.key.includes('/');
   }

   /**
    * Whether category contains subcategories.
    */
   get isParent() {
      return this.subcategories.size > 0;
   }

   jsonLD(): JsonLD.Blog | JsonLD.WebPage {
      return forCategory(this);
   }
}
