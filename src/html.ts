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

/**
 * Format Haiku text into three lines.
 */
export function formatHaiku(text: string, regex: RegExp): string {
  const match = regex.exec(text);

  if (match === null) {
    return text;
  }

  return (
    '<p class="haiku">' +
    match[1] +
    "<br/>" +
    match[2] +
    "<br/>" +
    match[3] +
    iconTag("spa") +
    "</p>" +
    caption(text.replace(match[0], ""))
  );
}

/**
 * Convert new lines to HTML paragraphs and normalize links.
 *
 * @see https://developer.mozilla.org/en-US/docs/JavaScript/Reference/Global_Objects/String/replace
 */
function caption(text: string): string {
  if (!is.empty(text)) {
    const ph = "[POEM]"; // poetry placeholder
    let footnotes = "";
    let poem = "";

    text = fixMalformedLink(text);
    text = shortenLinkText(text);
    text = typography(text);

    text = text
      // format footnotes separately
      .replace(
        re.footnote.text,
        (_match: string, _prefix: string, body: string) => {
          footnotes = formatNotes(body);
          return "";
        }
      )
      // set poetry aside and replace with placeholder
      .replace(
        re.poetry.any,
        (_match: string, _space: string, body: string) => {
          poem = formatPoem(body);
          return ph;
        }
      )
      // remove block quotes and wrap in fake tags that won't match
      // subsequent operations
      .replace(
        re.quote.block,
        (_match: string, _newLines: string, body: string) =>
          "[Q]" + body.replace(re.quote.curly, "") + "[/Q]"
      );

    text = "<p>" + text + "</p>";

    text = text
      .replace(re.newLine, "</p><p>")
      .replace(re.tag.emptyParagraph, "")
      .replace(
        re.quip,
        (_match, _tag: string, body: string) => '<p class="quip">' + body
      )
      .replace(re.footnote.number, "$1<sup>$2</sup>")
      // restore blockquotes
      .replace(/\[\/Q][\r\n\s]*([^<]+)/g, '[/Q]<p class="first">$1')
      .replace(/(<p>)?\[Q]/g, "<blockquote><p>")
      .replace(/\[\/Q](<\/p>)?/g, "</p></blockquote>");

    if (poem.length > 0) {
      text = text
        .replace(ph, "</p>" + poem + '<p class="first">')
        .replace(re.tag.emptyParagraph, "");
    }
    return text + footnotes;
  }
  return "";
}
