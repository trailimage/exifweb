import { lipsum } from "@toba/test";
import { html } from "./html";
import { config } from "../config/";

/** New-line */
const nl = "\r\n";
/** Double-space */
const ds = nl + nl;
const empty = "";


it("formats blockquote with trailing ellipsis", () => {
  const phrase =
    "Firefighters are working to get a handle on several wildfires that sparked during a lightning storm on Thursday night. Strong winds and poor visibility created challenges for firefighters working the blazes on Saturday ...";
  const source = lipsum + ds + "“" + phrase + "”¹" + ds + lipsum;
  const target =
    "<p>" +
    lipsum +
    "</p><blockquote><p>" +
    phrase +
    '<sup>¹</sup></p></blockquote><p class="first">' +
    lipsum +
    "</p>";

  expect(html.caption(source)).toBe(target);
});

it("formats block quote when it is the entire caption", () => {
  const source = "“" + lipsum + "”¹";
  const target = "<blockquote><p>" + lipsum + "<sup>¹</sup></p></blockquote>";
  expect(html.caption(source)).toBe(target);
});

// “The historic, 101-mile, single-lane, mostly-unimproved Magruder Corridor Road winds through a vast undeveloped area, offering solitude and pristine beauty as well as expansive mountain views. The corridor was created in 1980 leaving a unique road that enables a traveler to drive between two wildernesses: the 1.2 million-acre Selway-Bitterroot Wilderness to the north, and the 2.3-million-acre Frank Church-River of No Return Wilderness to the South. The road itself has changed little since its construction by the Civilian Conservation Corps (CCC) in the 1930s.”¹
// ___
// ¹ U.S. Forest Service, “Magruder Road Corridor”: https://www.fs.usda.gov/recarea/nezperceclearwater/recarea/?recid=16482

it("does not blockquote interrupted quotes", () => {
  // do no blockquote when quote is interrupted
  // “The constitutions of nearly all the states have qualifications for voters simply on citizenship,” Pefley countered, “without question with regard to what they believe on this or that question. Then I ask, why make a distinction of the people of Idaho?
  // “It appears to have been reserved for Idaho’s constitution to put in the first religious test in regard to the right of suffrage and holding office … Political and religious persecution are supposed to have died at the termination of the revolution but it appears that Idaho is again an exception.”¹
  // Pefley’s arguments were unheeded and the section was approved.

  const source =
    "“" + lipsum + ",” he said, “" + lipsum + ds + "“" + lipsum + "”" + ds;
  const target =
    "<p>“" +
    lipsum +
    ",” he said, “" +
    lipsum +
    "</p><blockquote><p>" +
    lipsum +
    "</p></blockquote>";

  expect(html.caption(source)).toBe(target);
});

it("formats inline poems", () => {
  const poemText = `Have you ever stood on the top of a mountain
And gazed down on the grandeur below
And thought of the vast army of people
· · Who never get out as we go?

Have you ever trailed into the desert
Where the hills fade from gold into blue,
And then thought of some poor other fellow
Who would like to stand alongside of you?`;

  const poemHTML =
    '<blockquote class="poem"><p>' +
    "Have you ever stood on the top of a mountain<br/>" +
    "And gazed down on the grandeur below<br/>" +
    "And thought of the vast army of people<br/>" +
    '<span class="tab"></span>Who never get out as we go?</p><p>' +
    "Have you ever trailed into the desert<br/>" +
    "Where the hills fade from gold into blue,<br/>" +
    "And then thought of some poor other fellow<br/>" +
    "Who would like to stand alongside of you?</p></blockquote>";

  // no text after
  let source = lipsum + ds + poemText;
  let target = "<p>" + lipsum + "</p>" + poemHTML;

  expect(html.caption(source)).toBe(target);

  // text after poem
  source = lipsum + ds + poemText + ds + lipsum;
  target =
    "<p>" + lipsum + "</p>" + poemHTML + '<p class="first">' + lipsum + "</p>";

  expect(html.caption(source)).toBe(target);
});

it("formats footnoted poems", () => {
  const source = `Now many years have passed since we lived there and little connects us to that place—now in other hands—other than our shared memories. My mom has written of Our Old House:

“When I drive by I always think I see myself
standing in the large picture window waving,
wishing I’d stop by and have a spot of tea.

“But I know its only what I want
because I didn’t want to leave, you see,
and when I drive by, smell the row
of lilacs I planted along the road,
see the gray smoke curling from the chimney,

“I want to pull in and stop,
pretend I never left, unload the groceries,
stoke the fire, straighten the photos on the wall
and wash the dishes that have stacked
by the sink for the last ten years.

“You’d be there, too, in your blue pajamas
asking for a story. We’d climb the narrow
staircase to your room and turn on the lamp,
listening for a moment to the frogs outside,
that bellowed thousands strong.

“I’d read your Sweet Pickles books¹
and sing that Bumble Bee song you loved.
Then we’d lay quietly and never grow old,
while time went on without us, down
the dusty country road, slipping over the horizon,
leaving a soft orange glow for us to read by.”²

In recent years I’ve tried to make the annual, three-hundred mile pilgrimage to “Troy Days.”³ Starchy pancake-feed food, a couple fire trucks and horses paraded down main street, and an evening of under-age inebriation make a good time, of course, but my trip is not for those things. Troy Days is when and where my dad’s brothers reunite annually from their homes across the western U.S. In their company, my mind can visit our old house, find a place alongside my dad, my grandma and the rest seated around a fire, our eyes all reflecting the same eternal glow.

This particular weekend had an additional attraction, my nephew Kaden’s seventh birthday party. I don’t see my nephews often so I was glad for the coincidence of events.
___
¹ Wikipedia: http://en.wikipedia.org/wiki/Sweet_Pickles
² Cheryl Reed, January 17, 2003: http://www.amazon.com/Cheryl-Dudley/e/B001JP7LNO/ref=ntt_athr_dp_pel_1`;

  const target =
    "<p>Now many years have passed since we lived there and little connects " +
    "us to that place—now in other hands—other than our shared memories. My " +
    "mom has written of Our Old House:</p>" +
    '<blockquote class="poem"><p>' +
    "When I drive by I always think I see myself" +
    "<br/>standing in the large picture window waving," +
    "<br/>wishing I’d stop by and have a spot of tea." +
    "</p><p>" +
    "But I know its only what I want" +
    "<br/>because I didn’t want to leave, you see," +
    "<br/>and when I drive by, smell the row" +
    "<br/>of lilacs I planted along the road," +
    "<br/>see the gray smoke curling from the chimney," +
    "</p><p>" +
    "I want to pull in and stop," +
    "<br/>pretend I never left, unload the groceries," +
    "<br/>stoke the fire, straighten the photos on the wall" +
    "<br/>and wash the dishes that have stacked" +
    "<br/>by the sink for the last ten years." +
    "</p><p>" +
    "You’d be there, too, in your blue pajamas" +
    "<br/>asking for a story. We’d climb the narrow" +
    "<br/>staircase to your room and turn on the lamp," +
    "<br/>listening for a moment to the frogs outside," +
    "<br/>that bellowed thousands strong." +
    "</p><p>" +
    "I’d read your Sweet Pickles books<sup>¹</sup>" +
    "<br/>and sing that Bumble Bee song you loved." +
    "<br/>Then we’d lay quietly and never grow old," +
    "<br/>while time went on without us, down" +
    "<br/>the dusty country road, slipping over the horizon," +
    "<br/>leaving a soft orange glow for us to read by.<sup>²</sup>" +
    "</p></blockquote>" +
    '<p class="first">' +
    "In recent years I’ve tried to make the annual, three-hundred mile " +
    "pilgrimage to “Troy Days.”<sup>³</sup> Starchy pancake-feed food, a couple fire " +
    "trucks and horses paraded down main street, and an evening of under-age " +
    "inebriation make a good time, of course, but my trip is not for those " +
    "things. Troy Days is when and where my dad’s brothers reunite annually " +
    "from their homes across the western U.S. In their company, my mind can " +
    "visit our old house, find a place alongside my dad, my grandma and the " +
    "rest seated around a fire, our eyes all reflecting the same eternal " +
    "glow." +
    "</p><p>" +
    "This particular weekend had an additional attraction, my nephew Kaden’s " +
    "seventh birthday party. I don’t see my nephews often so I was glad for " +
    "the coincidence of events." +
    "</p>" +
    '<ol class="footnotes">' +
    "<li><span>Wikipedia: http://en.wikipedia.org/wiki/Sweet_Pickles</span></li>" +
    "<li><span>Cheryl Reed, January 17, 2003: http://www.amazon.com/Cheryl-Dudley/e/B001JP7LNO/ref=ntt_athr_dp_pel_1</span></li>" +
    "</ol>";

  expect(html.caption(source)).toBe(target);
});

it("formats haiku", () => {
  let source =
    "neck bent" + nl + "apply the brakes" + nl + "for the reign of fire";
  let target =
    '<p class="haiku">neck bent<br/>apply the brakes<br/>for the reign of fire<i class="material-icons spa">spa</i></p>';

  expect(html.story(source)).toBe(target);

  source =
    "cows stand chewing" +
    nl +
    "wet meadow grass" +
    nl +
    "while mud swallows wheels" +
    ds +
    'Here we have Joel "Runs with Cows" Abbott. He did a little loop out among them—kind of became one of them.';
  target =
    '<p class="haiku">cows stand chewing<br/>wet meadow grass<br/>while mud swallows wheels<i class="material-icons spa">spa</i></p>' +
    "<p>Here we have Joel &ldquo;Runs with Cows&rdquo; Abbott. He did a little loop out among them—kind of became one of them.</p>";

  expect(html.story(source)).toBe(target);
});

it("does not convert conversation to a poem", () => {
  const source =
    "“What’s wrong Brenna?” I ask." +
    ds +
    "“I can’t sleep.”" +
    ds +
    "“Just lay down.”" +
    ds +
    "“I can’t.”" +
    ds +
    "“Brenna,” I insist, “lay down.”";

  const target =
    '<p class="quip">“What’s wrong Brenna?” I ask.</p>' +
    "<p>“I can’t sleep.”</p>" +
    "<p>“Just lay down.”</p>" +
    "<p>“I can’t.”</p>" +
    "<p>“Brenna,” I insist, “lay down.”</p>";

  expect(html.story(source)).toBe(target);
});

it("formats captions that are entirely a poem", () => {
  const source =
    "-" +
    nl +
    "Begotten Not Born" +
    nl +
    "Indwelling Transcendence" +
    nl +
    "· · · · Infinite Regress" +
    nl +
    "Uncertain Progress" +
    nl +
    "-";
  const target =
    '<p class="poem">' +
    "Begotten Not Born<br/>" +
    "Indwelling Transcendence<br/>" +
    '<span class="tab"></span><span class="tab"></span>Infinite Regress<br/>' +
    "Uncertain Progress</p>";

  expect(html.story(source)).toBe(target);
});

it("styles superscripts", () => {
  const source = lipsum + "²";
  const target = "<p>" + lipsum + "<sup>²</sup></p>";
  expect(html.caption(source)).toBe(target);
});

it("formats footnotes", () => {
  let source =
    lipsum +
    nl +
    "___" +
    nl +
    "* Note about photo credit" +
    nl +
    "¹ Some other note" +
    nl +
    "² Last note";
  let target =
    "<p>" +
    lipsum +
    '</p><ol class="footnotes" start="0">' +
    '<li class="credit"><i class="material-icons star">star</i><span>Note about photo credit</span></li>' +
    "<li><span>Some other note</span></li>" +
    "<li><span>Last note</span></li></ol>";

  expect(html.caption(source)).toBe(target);

  source = lipsum + nl + "___" + nl + "¹ Some other note" + nl + "² Last note";
  target =
    "<p>" +
    lipsum +
    '</p><ol class="footnotes">' +
    "<li><span>Some other note</span></li>" +
    "<li><span>Last note</span></li></ol>";

  expect(html.caption(source)).toBe(target);

  // should ignore trailing newline
  source += nl;

  expect(html.caption(source)).toBe(target);
});
