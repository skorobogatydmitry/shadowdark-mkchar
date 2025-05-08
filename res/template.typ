
// #import "inputs-sample.typ": inputs
#import sys: inputs

#set page(paper: "a6", margin: (x: 2%, y: 1%))

#stack(
  dir: ltr,
  spacing: 2%,
  box(
    width: 25%,
    align(
      top,
      [
        #set text(8pt, font: "Brahms Gotisch Cyr")
        = #inputs.terms.name
        #set text(18pt, font: "Kereru")
        #inputs.name
        #set text(8pt, font: "Brahms Gotisch Cyr")
        = #inputs.terms.ancestry
        #set text(18pt, font: "Kereru")
        #inputs.ancestry
        #set text(8pt, font: "Brahms Gotisch Cyr")
        = #inputs.terms.language
        #set text(12pt, font: "Kereru")
        #for l in inputs.languages [
          - #l
        ]
        #set par(spacing: .2em)
        #table(
          stroke: 1pt,
          columns: (.7fr, 1fr),
          align: center,
          [
            #set text(12pt, font: "Brahms Gotisch Cyr")
            #inputs.terms.hit_points
          ],
          [
            #set text(18pt, font: "Kereru")
            #inputs.hit_points
          ],
        )
        #set text(8pt, font: "Brahms Gotisch Cyr")
        = #inputs.terms.stats
        #table(
          columns: (1fr, 1fr),
          stroke: none,
          inset: 15%,
          align: center,
          ..for stat in inputs.stats {
            (
              table.cell(
                colspan: 2,
                inset: (top: 30%),
                [
                  #set text(8pt, font: "Brahms Gotisch Cyr")
                  = #stat.name
                ],
              ),
              table.hline(stroke: 1pt),
              [
                #set text(18pt, font: "Kereru")
                #stat.value
              ],
              [
                #set text(18pt, font: "Kereru")
                #stat.modifier
              ],
            )
          },
          table.vline(stroke: 1pt, x: 1),
        )
      ],
    ),
  ),
  box(
    width: 40%,
    align(
      top,
      [
        #set text(12pt, font: "Brahms Gotisch Cyr")
        #set align(center)
        = #inputs.terms.talent
        #set text(6pt, font: "Kereru")
        #table(
          columns: 1fr,
          stroke: none,
          inset: .5em,
          align: center,
          ..for t in inputs.class_features {
            (
              [
                #set text(8pt, font: "Kereru")
                = #t.name
              ],
              table.cell(
                stroke: 1pt,
                align: left,
                inset: (y: .7em, x: .5em),
                [
                  #set text(10pt, font: "Kereru")
                  #t.description
                ],
              ),
            )
          }
        )
        #set text(6pt, font: "Kereru")
        #table(
          columns: 1fr,
          stroke: none,
          inset: .5em,
          align: center,
          [
            #set text(8pt, font: "Kereru")
            = #inputs.ancestry_feature.name
          ],
          table.cell(
            stroke: 1pt,
            align: left,
            inset: (y: .7em, x: .5em),
            [
              #set text(10pt, font: "Kereru")
              #inputs.ancestry_feature.description
            ],
          ),
        )
      ],
    ),
  ),
  box(
    width: 30%,
    align(
      top,
      [
        #set text(8pt, font: "Brahms Gotisch Cyr")
        = #inputs.terms.background
        #set text(14pt, font: "Kereru")
        #inputs.background
        #set text(8pt, font: "Brahms Gotisch Cyr")
        = #inputs.terms.alignment
        #set text(12pt, font: "Kereru")
        #inputs.alignment
        #set text(8pt, font: "Brahms Gotisch Cyr")
        = #inputs.terms.deity
        #set text(18pt, font: "Kereru")
        #inputs.deity
        #set text(8pt, font: "Brahms Gotisch Cyr")
        = #inputs.terms.class
        #set text(18pt, font: "Kereru")
        #inputs.class
        #set text(8pt, font: "Brahms Gotisch Cyr")
        = #inputs.terms.weapon
        #set text(12pt, font: "Kereru")
        #for wm in inputs.weapon_masteries [
          - #wm
        ]
        #set text(8pt, font: "Brahms Gotisch Cyr")
        = #inputs.terms.armor
        #set text(12pt, font: "Kereru")
        #for am in inputs.armor_masteries [
          - #am
        ]
      ],
    ),
  ),
)

#pagebreak()

#let no_spells = inputs.spells.len() == 0

#let inventory_width = if no_spells { 100% } else { 40% }

#let spells_box = if no_spells {
  box()
} else {
  box(
    width: 58%,
    [
      #set align(center)
      = #inputs.terms.spell
      #set text(6pt, font: "Kereru")
      #table(
        columns: 1fr,
        stroke: none,
        inset: .5em,
        align: center,
        ..for t in inputs.spells {
          (
            [
              #set text(8pt, font: "Kereru")
              = #t.name
            ],
            table.cell(
              stroke: 1pt,
              align: left,
              inset: (y: .7em, x: .5em),
              [
                #set text(10pt, font: "Kereru")
                #t.description
              ],
            ),
          )
        }
      )
    ],
  )
}

#stack(
  dir: ltr,
  spacing: 2%,
  box(
    width: inventory_width,
    [
      #set text(18pt, font: "Kereru")
      #table(
        columns: (auto, 1fr),
        align: left,
        stroke: none,
        inset: (x: 1pt, y: 2pt),
        table.cell(
          stroke: 1pt,
          colspan: 2,
          align: center,
          [
            #set text(12pt, font: "Brahms Gotisch Cyr")
            = #inputs.terms.inventory
          ],
        ),
        ..for i in range(inputs.slots_count) {
          (
            table.cell(align: right, [#(i + 1)]),
            table.hline(stroke: .5pt),
            table.vline(stroke: .5pt),
            [#inputs.items.at(i, default: "")],
          )
        },
      )
      #set text(18pt, font: "Kereru")
      #table(
        columns: (.7fr, 1fr, .7fr, 1fr),
        align: center,
        stroke: 1pt,
        inset: (y: .3em),
        table.cell(
          colspan: 4,
          [
            #set text(12pt, font: "Brahms Gotisch Cyr")
            = #inputs.terms.purse
          ],
        ),
        table.cell(inset: (y: .3em), [#inputs.terms.gold_pieces]),
        table.cell(inset: (y: .3em), align: left, [#inputs.gold_pieces]),
        table.cell(inset: (y: .3em), [#inputs.terms.silver_pieces]),
        table.cell(inset: (y: .3em), align: left, [#inputs.silver_pieces]),
      )
    ],
  ),
  spells_box,
)
