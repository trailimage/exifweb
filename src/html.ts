import { is, format, LinkRelation } from "@toba/node-tools";
import { Category } from "@trailimage/models";
import { re } from "../regex";
import { config } from "../config";
import { htmlEntity } from "./constants";

/**
 * Format paragraphs and prose.
 */
export function story(text: string): string {
  if (is.empty(text)) {
    return text;
  }

  if (re.poetry.all.test(text)) {
    // text is entirely a poem or haiku
    text = text.replace(re.poetry.delimiter, "");

    if (re.haiku.all.test(text)) {
      // haiku
      text = formatHaiku(text, re.haiku.all);
    } else {
      // not hiaku
      text =
        '<p class="poem">' +
        text
          .replace(re.lineBreak, "<br/>")
          .replace(re.poetry.indent, '<span class="tab"></span>') +
        "</p>";
    }
  } else if (re.haiku.any.test(text)) {
    // text begins with a haiku
    text = formatHaiku(text, re.haiku.any);
  } else {
    // text has no haiku but may be partially a poem
    text = caption(text);
  }

  return text;
}

/**
 * If link text is a web address, replace with just domain and page.
 */
export const shortenLinkText = (text: string) =>
  text.replace(re.tag.linkToUrl, (_match, protocol, url: string) => {
    const parts = url.split("/");
    const domain = parts[0].replace("www.", "");
    // page precedes trailing slash
    let lastPart = /\/$/.test(url) ? parts.length - 2 : parts.length - 1;
    // if last part is only a query string then move to previous
    if (lastPart > 0 && /^[\?#]/.test(parts[lastPart])) {
      lastPart--;
    }

    let middle = "/";
    const page = parts[lastPart]
      .replace(re.queryString, "")
      .replace(re.tag.anchor, "")
      .replace(re.fileExt, "");

    if (lastPart > 1) {
      middle = "/&hellip;/";
    }
    if (protocol === undefined) {
      protocol = "http://";
    }

    return (
      '<a href="' +
      protocol +
      url +
      '">' +
      domain +
      middle +
      decodeURIComponent(page) +
      "</a>"
    );
  });

export function linkPattern(url: string): string {
  return `<a href="${url}$1" target="_blank">$1</a>`;
}
