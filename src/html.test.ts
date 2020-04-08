it("formats inline poems", () => {


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
