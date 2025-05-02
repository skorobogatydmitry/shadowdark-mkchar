
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
        #set text(4pt, font: "Brahms Gotisch Cyr")
        = #inputs.terms.name
        #set text(10pt, font: "Kereru")
        #inputs.name
        #set text(4pt, font: "Brahms Gotisch Cyr")
        = #inputs.terms.ancestry
        #set text(10pt, font: "Kereru")
        #set par(spacing: .5em)
        #inputs.ancestry \
        #table(
          stroke: .8pt,
          columns: (.7fr, 1fr),
          align: center,
          [#inputs.terms.hit_points], [#inputs.hit_points],
        )
        #set text(4pt, font: "Brahms Gotisch Cyr")
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
                  #set text(7pt, font: "Kereru")
                  #stat.name
                ],
              ),
              table.hline(stroke: .5pt),
              [
                #set text(10pt, font: "Kereru")
                #stat.value
              ],
              [
                #set text(10pt, font: "Kereru")
                #stat.modifier
              ],
            )
          },
          table.vline(stroke: .5pt, x: 1),
        )
        #set text(4pt, font: "Brahms Gotisch Cyr")
        = #inputs.terms.language
        #set text(6pt, font: "Kereru")
        #inputs.languages.join("\n")
        #set text(6pt, font: "Kereru")
        #table(
          columns: 1,
          stroke: none,
          inset: 15%,
          align: center,
          [
            #set text(7pt, font: "Kereru")
            #inputs.ancestry_feature.name
          ],
          table.cell(
            stroke: .5pt,
            align: left,
            inset: (y: 10%, x: 3%),
            [
              #inputs.ancestry_feature.description],
          ),
        )
      ],
    ),
  ),
  box(
    width: 40%,
    align(
      top,
      [
        #set text(4pt, font: "Brahms Gotisch Cyr")
        = #inputs.terms.background
        #set text(10pt, font: "Kereru")
        #inputs.background
        #set text(4pt, font: "Brahms Gotisch Cyr")
        = #inputs.terms.alignment
        #set text(10pt, font: "Kereru")
        #inputs.alignment
        #set text(4pt, font: "Brahms Gotisch Cyr")
        = #inputs.terms.deity
        #set text(8pt, font: "Kereru")
        #inputs.deity
        #set text(6pt, font: "Brahms Gotisch Cyr")
        #set align(center)
        = #inputs.terms.talent
        #set text(6pt, font: "Kereru")
        #table(
          columns: 1,
          stroke: none,
          inset: 15%,
          align: center,
          ..for t in inputs.class_features {
            (
              [
                #set text(7pt, font: "Kereru")
                #t.name
              ],
              table.cell(
                stroke: .5pt,
                align: left,
                inset: (y: 10%, x: 3%),
                [
                  #t.description],
              ),
            )
          }
        )
      ],
    ),
  ),
  box(
    width: 30%,
    align(
      top,
      [
        #set text(4pt, font: "Brahms Gotisch Cyr")
        = #inputs.terms.class
        #set text(10pt, font: "Kereru")
        #inputs.class
        #set text(4pt, font: "Brahms Gotisch Cyr")
        = #inputs.terms.weapon
        #set text(8pt, font: "Kereru")
        #inputs.weapon_masteries.join("\n")
        #set text(4pt, font: "Brahms Gotisch Cyr")
        = #inputs.terms.armor
        #set text(8pt, font: "Kereru")
        #set par(spacing: .5em)
        #inputs.armor_masteries.join("\n")
        #set text(6pt, font: "Kereru")
        #table(
          columns: (.7fr, 1fr, .7fr, 1fr),
          align: center,
          stroke: .5pt,
          inset: 15%,
          table.cell(
            colspan: 4,
            [
              #set text(8pt, font: "Brahms Gotisch Cyr")
              #inputs.terms.purse],
          ),
          table.cell(inset: (x: 5%, y: 30%), [#inputs.terms.gold_pieces]),
          table.cell(inset: (x: 5%, y: 30%), [#inputs.gold_pieces]),
          table.cell(inset: (x: 5%, y: 30%), [#inputs.terms.silver_pieces]),
          table.cell(inset: (x: 5%, y: 30%), [#inputs.silver_pieces]),
        )
        #table(
          columns: (auto, 1fr),
          align: left,
          stroke: none,
          inset: (x: 1pt, y: 2pt),
          table.cell(
            stroke: .5pt,
            colspan: 2,
            align: center,
            [
              #set text(8pt, font: "Brahms Gotisch Cyr")
              #inputs.terms.inventory],
          ),
          ..for i in range(inputs.slots_count) {
            (
              table.hline(stroke: .5pt),
              table.cell(align: right, [#(i + 1)]),
              table.vline(stroke: .5pt),
              [#inputs.items.at(i, default: "")],
            )
          }
        )
      ],
    ),
  ),
)
