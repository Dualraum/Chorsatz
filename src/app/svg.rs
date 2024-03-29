use leptos::*;

/// Creates an svg for a musical staff with bass and violin line, clefs. Is open to the right.
#[component]
fn FullStaff(width: f32) -> impl IntoView {
    view! {
        <title>Grand Staff</title>
        <desc>Full Musical Staff by Linus Mußmächer, brace and clefs taken from commons.wikimedia.org/wiki/Category:SVG_musical_notation</desc>

        <g id="brace_group" transform="translate(2.0 20.0)">
        <g transform="scale(1.21)">
        <path
            id="brace"
            transform="matrix(0.04,0,0,-0.04,8.52,66.119983)"
            d="m -15,-498 c 0,-261 -102,-516 -102,-765 0,-135 30,-261 129,-360 3,-3 3,-9 3,-12 0,-9 -6,-18 -15,-18 -3,0 -9,3 -12,6 -132,135 -177,312 -177,501 0,267 108,528 108,780 0,135 -33,261 -129,360 -3,3 -3,3 -3,6 0,3 0,3 3,6 96,99 129,225 129,360 0,252 -108,513 -108,780 0,189 45,366 177,501 3,3 9,6 12,6 9,0 15,-9 15,-18 0,-3 0,-9 -3,-12 -99,-99 -129,-225 -129,-360 0,-249 102,-504 102,-765 C -15,312 -60,135 -189,0 -60,-135 -15,-312 -15,-498 z"
            style="fill:rgb(0,0,0)" />
        </g></g>

        <g id="bass_clef_group" transform="translate(25.0 140.0)">
        <g transform="scale(0.038)">
        <g transform="matrix(1 0 0 -1 0 0)">
            <path
            id="bass_clef"
            d="M567 -386c0,29,23,52,52,52s52,-23,52,-52s-23,-52,-52,-52s-52,23,-52,52z
            m0,250c0,29,23,52,52,52s52,-23,52,-52s-23,-52,-52,-52s-52,23,-52,52z
            m-323,136
            c171,0,292,-86,292,-248c0,-263,-264,-415,-517,-521c-2,-2,-5,-3,-8,-3c-6,0,-11,5,-11,11c0,3,1,6,3,8c203,118,415,265,415,494c0,121,-64,237,-174,237c-79,0,-138,-57,-164,-133
            c14,8,28,13,43,13c55,0,100,-45,100,-100c0,-58,-44,-107,-100,-107c-60,0,-112,48,-112,107c0,132,103,242,233,242z" />
        </g></g></g>

        <g id="violin_clef_group" transform="translate(20.0 -15.0)">
        <g transform="scale(0.091)">
        <path
            id="violin_clef"
            d="M 201.68015,653.12212 C 189.21313,655.67886 177.65942,662.51955 166.63177,673.41648 C 155.59034,684.51344 149.67988,697.17042 148.71367,711.17354 C 148.10635,719.9755 150.42417,730.18527 155.50802,741.18889 C 160.57805,752.39254 168.8631,760.80307 179.94828,766.59287 C 183.70316,767.65593 185.3699,769.78091 185.17666,772.58153 C 185.10766,773.58176 183.63491,774.48513 180.17068,775.05009 C 162.23985,769.18995 147.87638,758.35004 137.25321,742.94427 C 126.64383,727.33847 121.78734,710.11963 122.71136,690.88769 C 124.73465,670.32462 132.27618,651.54928 145.32214,634.7617 C 158.58245,617.78793 174.89289,606.25054 194.2535,600.14951 L 185.31689,528.179 C 152.66838,551.65388 125.76626,576.53025 104.38237,603.19434 C 83.012279,629.65841 70.901511,659.1733 67.84952,691.72521 C 67.242971,706.35613 69.270926,720.76683 73.947196,734.75728 C 78.609662,748.94777 86.176175,761.93166 96.61914,774.10905 C 117.71939,798.27758 146.38304,812.11416 182.19525,815.79114 C 194.48283,815.83498 207.66916,814.53387 221.95477,811.90162 L 201.68015,653.12212 z M 216.25638,652.11791 L 236.92479,808.1106 C 268.66231,797.83866 285.96656,771.89805 288.80996,730.68888 C 287.95758,716.76129 284.80274,704.0818 278.74385,692.6089 C 272.89929,680.94982 264.68326,671.53905 253.89522,664.3628 C 243.1072,657.18653 230.6946,653.11413 216.25638,652.11791 z M 189.50255,441.23515 C 196.39612,437.69086 204.49939,431.2151 213.39751,421.98021 C 222.2818,412.94535 231.08979,402.09629 239.59334,389.81922 C 248.31123,377.35594 255.43867,364.58193 260.97568,351.4972 C 266.49887,338.6125 269.5743,326.16189 270.37486,314.55931 C 270.71993,309.5582 270.66394,304.52941 269.96496,300.05925 C 269.65973,292.8023 267.84037,287.04887 264.29255,282.98512 C 260.73091,279.12142 256.0562,276.78891 250.04029,276.37381 C 238.00843,275.54364 226.66906,282.19811 216.02218,296.33725 C 207.71915,308.62816 200.45368,323.40261 194.85495,340.30203 C 189.0419,357.38766 185.24795,374.41163 183.64604,391.78782 C 183.28222,411.66141 185.37268,428.0864 189.50255,441.23515 z M 176.06855,451.96604 C 169.28073,418.93619 166.54493,385.58293 167.86115,351.90628 C 169.5524,330.31532 173.15233,310.26302 178.66095,291.74939 C 183.96903,273.22194 190.90246,257.41959 199.48882,243.94227 C 207.87467,230.4511 217.22903,220.44371 227.35143,213.90624 C 236.41593,208.09979 242.86704,205.12796 246.27607,205.36318 C 248.88296,205.54305 251.01979,206.69548 252.90087,208.63425 C 254.78195,210.57302 257.18181,213.75355 260.11425,217.97582 C 281.64472,253.42988 290.86079,295.06914 287.5757,342.67973 C 286.01598,365.28476 281.48971,387.08211 273.95546,408.67191 C 266.63557,430.0755 256.57724,450.28516 243.8081,468.9008 C 230.82464,487.70266 216.0153,503.76557 199.16576,517.27574 L 209.97043,597.21402 C 218.86279,596.8226 224.93393,596.43752 228.34295,596.67273 C 243.5833,597.7243 256.99854,601.8659 269.19028,609.13901 C 281.38203,616.41214 291.61718,625.76121 299.68137,637.37246 C 307.75937,648.78367 313.72164,661.65687 317.56818,675.99205 C 321.21419,690.3134 322.81348,705.09653 321.76446,720.2999 C 320.13573,743.90516 312.42856,765.08103 298.65677,783.62749 C 284.88499,802.17395 265.25562,815.09032 239.55434,822.56283 C 240.48225,832.47569 242.30967,846.87257 245.26476,865.3672 C 248.0055,884.048 250.00586,898.8588 251.26582,909.7995 C 252.52577,920.7403 252.61014,931.198 251.90619,941.4003 C 250.81577,957.2039 246.03949,970.944 237.56353,982.821 C 228.88706,994.6841 217.79692,1003.5667 204.0926,1009.455 C 190.58879,1015.3572 175.91593,1017.7617 160.27452,1016.6824 C 138.21614,1015.1604 119.39308,1007.6308 103.79152,994.2935 C 88.203778,980.7562 80.526062,963.5437 81.187044,942.2835 C 82.437365,932.923 85.250533,924.2733 89.840868,916.1481 C 94.431205,908.023 100.31483,901.5951 107.69229,896.8782 C 114.88301,891.9474 123.29802,889.7141 132.76437,889.7643 C 140.58507,890.3039 147.85288,893.0163 154.58159,897.7016 C 161.09597,902.573 166.31056,908.7616 170.011,916.4539 C 173.5109,924.1323 175.15082,932.4863 174.5435,941.2882 C 173.72913,953.0909 169.02838,962.8163 160.44122,970.4647 C 151.85408,978.113 141.33714,981.6084 129.10477,980.7643 L 124.49256,980.4461 C 131.48509,992.9884 143.89116,1000.0753 161.73841,1001.3067 C 170.76229,1001.9293 180.12474,1000.5654 189.59761,997.6011 C 199.2848,994.4505 207.45058,989.989 214.50978,984.0442 C 221.569,978.0993 226.44997,971.6022 228.75164,964.5252 C 232.72659,956.5586 235.1176,945.2668 236.0976,931.0636 C 236.76014,921.4615 236.42002,911.7902 235.27778,902.0635 C 234.12174,892.5369 232.18389,879.74039 229.45043,863.874 C 226.70317,848.2077 224.72391,836.01129 223.68559,827.6988 C 211.44668,829.86928 198.90336,830.61179 185.86885,829.71243 C 164.01101,828.20426 143.65999,822.37811 124.82964,812.03398 C 105.99928,801.68985 89.809943,788.11099 76.074886,771.08354 C 62.540374,754.06992 52.352377,735.27724 45.538506,714.3054 C 38.911364,693.54744 36.163324,672.05217 37.481102,650.03344 C 39.691119,629.68424 44.853882,610.34277 53.342859,592.43679 C 61.845634,574.33074 72.2847,557.36331 84.846791,541.74834 C 97.40888,526.13337 110.27541,511.94638 123.43258,499.38743 C 136.77648,487.04234 154.19272,471.15931 176.06855,451.96604 z "
            style="fill:#131516;fill-rule:evenodd;stroke:#131516;stroke-width:0.60229105"
            />
        </g></g>

        <g id="upper_lines">
            <line id="upper_line_01" vector-effect="non-scaling-stroke" x1="15.0" y1=" 20.0" x2={width+15.+40.} y2=" 20.0" stroke=" rgb(0,0,0)" stroke-dasharray=" none" stroke-linecap=" butt" stroke-dashoffset="0" stroke-linejoin=" butt" stroke-miterlimit="4" fill=" rgb(0,0,0)" fill-rule=" nonzero"/>
            <line id="upper_line_02" vector-effect="non-scaling-stroke" x1="15.0" y1=" 30.0" x2={width+15.+40.} y2=" 30.0" stroke=" rgb(0,0,0)" stroke-dasharray=" none" stroke-linecap=" butt" stroke-dashoffset="0" stroke-linejoin=" butt" stroke-miterlimit="4" fill=" rgb(0,0,0)" fill-rule=" nonzero"/>
            <line id="upper_line_03" vector-effect="non-scaling-stroke" x1="15.0" y1=" 40.0" x2={width+15.+40.} y2=" 40.0" stroke=" rgb(0,0,0)" stroke-dasharray=" none" stroke-linecap=" butt" stroke-dashoffset="0" stroke-linejoin=" butt" stroke-miterlimit="4" fill=" rgb(0,0,0)" fill-rule=" nonzero"/>
            <line id="upper_line_04" vector-effect="non-scaling-stroke" x1="15.0" y1=" 50.0" x2={width+15.+40.} y2=" 50.0" stroke=" rgb(0,0,0)" stroke-dasharray=" none" stroke-linecap=" butt" stroke-dashoffset="0" stroke-linejoin=" butt" stroke-miterlimit="4" fill=" rgb(0,0,0)" fill-rule=" nonzero"/>
            <line id="upper_line_05" vector-effect="non-scaling-stroke" x1="15.0" y1=" 60.0" x2={width+15.+40.} y2=" 60.0" stroke=" rgb(0,0,0)" stroke-dasharray=" none" stroke-linecap=" butt" stroke-dashoffset="0" stroke-linejoin=" butt" stroke-miterlimit="4" fill=" rgb(0,0,0)" fill-rule=" nonzero"/>
        </g>

        <g id="lower_lines">
            <line id="lower_line_06" vector-effect="non-scaling-stroke" x1="15.0" y1="140.0" x2={width+15.+40.} y2="140.0" stroke=" rgb(0,0,0)" stroke-dasharray=" none" stroke-linecap=" butt" stroke-dashoffset="0" stroke-linejoin=" butt" stroke-miterlimit="4" fill=" rgb(0,0,0)" fill-rule=" nonzero"/>
            <line id="lower_line_07" vector-effect="non-scaling-stroke" x1="15.0" y1="150.0" x2={width+15.+40.} y2="150.0" stroke=" rgb(0,0,0)" stroke-dasharray=" none" stroke-linecap=" butt" stroke-dashoffset="0" stroke-linejoin=" butt" stroke-miterlimit="4" fill=" rgb(0,0,0)" fill-rule=" nonzero"/>
            <line id="lower_line_08" vector-effect="non-scaling-stroke" x1="15.0" y1="160.0" x2={width+15.+40.} y2="160.0" stroke=" rgb(0,0,0)" stroke-dasharray=" none" stroke-linecap=" butt" stroke-dashoffset="0" stroke-linejoin=" butt" stroke-miterlimit="4" fill=" rgb(0,0,0)" fill-rule=" nonzero"/>
            <line id="lower_line_09" vector-effect="non-scaling-stroke" x1="15.0" y1="170.0" x2={width+15.+40.} y2="170.0" stroke=" rgb(0,0,0)" stroke-dasharray=" none" stroke-linecap=" butt" stroke-dashoffset="0" stroke-linejoin=" butt" stroke-miterlimit="4" fill=" rgb(0,0,0)" fill-rule=" nonzero"/>
            <line id="lower_line_10" vector-effect="non-scaling-stroke" x1="15.0" y1="180.0" x2={width+15.+40.} y2="180.0" stroke=" rgb(0,0,0)" stroke-dasharray=" none" stroke-linecap=" butt" stroke-dashoffset="0" stroke-linejoin=" butt" stroke-miterlimit="4" fill=" rgb(0,0,0)" fill-rule=" nonzero"/>
        </g>

        <line id="left_line" vector-effect="non-scaling-stroke" x1="16.0" y1="20.0" x2="16.0" y2="180.0" stroke-width="2.0" stroke=" rgb(0,0,0)" stroke-dasharray=" none" stroke-linecap=" butt" stroke-dashoffset="0" stroke-linejoin=" butt" stroke-miterlimit="4" fill=" rgb(0,0,0)" fill-rule=" nonzero"/>
        <line id="right_line" vector-effect="non-scaling-stroke" x1={width+15.+40.-1.} y1="20.0" x2={width+15.+40.-1.} y2="180.0" stroke-width="2.0" stroke=" rgb(0,0,0)" stroke-dasharray=" none" stroke-linecap=" butt" stroke-dashoffset="0" stroke-linejoin=" butt" stroke-miterlimit="4" fill=" rgb(0,0,0)" fill-rule=" nonzero"/>

    }
}

/// Creates an svg view for a quarter note with the stem facing down.
#[component]
fn QuarterDown(head_center_x: f32, head_center_y: f32, index: usize) -> impl IntoView {
    let highlight = use_context::<ReadSignal<usize>>().expect("Could not unwrap highlight.");

    let col = move || {
        if highlight() == index {
            "rgb(22,132,227)"
        } else {
            "rgb(0,0,0)"
        }
    };
    view! {
        <g transform={format!("translate({} {})", head_center_x - 7., head_center_y -5.)}>
            <ellipse cx="7.0" cy="5.0" rx="7.0" ry="5.0" fill={col}/>
            <line id="stem" vector-effect="non-scaling-stroke" x1="1.0" y1="5.0" x2="1.0" y2="29.0" stroke={col} stroke-width="2.0" stroke-dasharray=" none" stroke-linecap=" round" stroke-dashoffset="0" stroke-linejoin=" round" stroke-miterlimit="4" />
        </g>
    }
}

/// Creates an svg view for a quarter note with the stem facing up.
#[component]
fn QuarterUp(head_center_x: f32, head_center_y: f32, index: usize) -> impl IntoView {
    let highlight = use_context::<ReadSignal<usize>>().expect("Could not unwrap highlight.");

    let col = move || {
        if highlight() == index {
            "rgb(22,132,227)"
        } else {
            "rgb(0,0,0)"
        }
    };
    view! {
        <g  transform={format!("translate({} {})", head_center_x - 7., head_center_y - 25.)}>
        <ellipse cx="7.0" cy="25.0" rx="7.0" ry="5.0" fill={col}/>
        <line id="stem" vector-effect="non-scaling-stroke" x1="13.0" y1="1.0" x2="13.0" y2="25.0" stroke={col} stroke-width="2.0" stroke-dasharray=" none" stroke-linecap=" round" stroke-dashoffset="0" stroke-linejoin=" round" stroke-miterlimit="4"/>
        </g>
    }
}

/// Creates an svg view for a sharp sign (hash symbol).
#[component]
fn Sharp(center_x: f32, center_y: f32) -> impl IntoView {
    view! {
        <g transform={format!("translate({} {})", center_x-5., center_y-12.)}>
        <g transform="scale(1.3)">
        <g transform="translate(-83, -435)">
        <path
        id="path2109"
        d="M 86.102,447.457 L 86.102,442.753 L 88.102,442.201 L 88.102,446.881 L 86.102,447.457 z M 90.04,446.319 L 88.665,446.713 L 88.665,442.033 L 90.04,441.649 L 90.04,439.705 L 88.665,440.089 L 88.665,435.30723 L 88.102,435.30723 L 88.102,440.234 L 86.102,440.809 L 86.102,436.15923 L 85.571,436.15923 L 85.571,440.986 L 84.196,441.371 L 84.196,443.319 L 85.571,442.935 L 85.571,447.606 L 84.196,447.989 L 84.196,449.929 L 85.571,449.545 L 85.571,454.29977 L 86.102,454.29977 L 86.102,449.375 L 88.102,448.825 L 88.102,453.45077 L 88.665,453.45077 L 88.665,448.651 L 90.04,448.266 L 90.04,446.319 z" />
        </g></g></g>
    }
}

/// Creates an svg view for a flat sign (b symbol).
#[component]
fn Flat(center_x: f32, center_y: f32) -> impl IntoView {
    view! {
        <g transform={format!("translate({} {})", center_x-5., center_y-12.)}>
        <g transform="scale(1.3)">
        <g transform="translate(-95, -435)">
            <path
            d="M 97.359,444.68428 C 96.732435,445.46734 96.205,445.91553 95.51,446.44253 L 95.51,443.848 C 95.668,443.449 95.901,443.126 96.21,442.878 C 96.518,442.631 96.83,442.507 97.146,442.507 C 98.621857,442.72115 98.104999,443.97562 97.359,444.68428 z M 95.51,442.569 L 95.51,435.29733 L 94.947,435.29733 L 94.947,446.91453 C 94.947,447.26653 95.043,447.44253 95.235,447.44253 C 95.346,447.44253 95.483913,447.34953 95.69,447.22653 C 97.091908,446.36314 97.992494,445.6642 98.89183,444.43098 C 99.16986,444.04973 99.366461,443.18512 98.96397,442.5813 C 98.71297,442.20474 98.234661,441.80922 97.621641,441.6923 C 96.828092,441.54095 96.14376,441.93605 95.51,442.569 z "
            style="fill:#000000"
            id="path2117" />
        </g></g></g>
    }
}

/// Creates an svg file for a note, composed of a quarter note with stem facing up or down and a number of correctly positioned signs (sharps or flats).
#[component]
fn Note(
    /// The octave and note name of the not to be displayed - influences relative y-position.
    note: crate::logic::notes::OctavedNote,
    /// Wether the note is displayed relative to the bass key in the lower part of the staff or the violin key in the upper part.
    move_to_lower_lines: bool,
    /// Wether the stem faces up or down.
    up: bool,
    /// x-position of the note.
    x: f32,
    /// Position of the sign relative to the note.
    #[prop(default = 0.)]
    sign_x: f32,
    /// An id to be attached to the svg.
    id: &'static str,
    index: usize,
) -> impl IntoView {
    let (line, sign) = note.get_note_line_and_sign();

    let (correct, lo, hi) = if move_to_lower_lines {
        (60., 140., 180.)
    } else {
        (0., 20., 60.)
    };

    let y = -line * 5. + correct + 105.;

    view! {
        <g id={format!("{} Note", id)}>
                // First Sharp
                {(sign >= 0.5).then(
                    || view!{<Sharp center_x={x-sign_x + 2. * SIGN_WIDTH} center_y={y} />}.into_view()
                )}
                // Second Sharp
                {(sign >= 1.0).then(
                    || view!{<Sharp center_x={x-sign_x+SIGN_WIDTH} center_y={y} />}.into_view()
                )}
                // First Flat
                {(sign <= -0.5).then(
                    || view!{<Flat center_x={x-sign_x+2. + 2. * SIGN_WIDTH} center_y={y} />}.into_view()
                )}
                // Second Flat
                {(sign <= -1.0).then(
                    || view!{<Flat center_x={x-sign_x+2. + SIGN_WIDTH} center_y={y} />}.into_view()
                )}
                // Helping Lines
                {(y > hi).then(
                    || ((hi/10.) as i32+1..=(y/10.).floor() as i32).map(|y|
                        view!{
                            <line id="helper_line" vector-effect="non-scaling-stroke" x1={x+25.} y1={10 * y} x2={x+45.} y2={10 * y} stroke=" rgb(0,0,0)" stroke-dasharray=" none" stroke-linecap=" butt" stroke-dashoffset="0" stroke-linejoin=" butt" stroke-miterlimit="4" />
                        }
                    ).collect_view()
                )}
                {(y < lo).then(
                    || ((y/10.).ceil() as i32..(lo/10.) as i32).map(|y|
                        view!{
                            <line id="helper_line" vector-effect="non-scaling-stroke" x1={x+25.} y1={10 * y} x2={x+45.} y2={10 * y} stroke=" rgb(0,0,0)" stroke-dasharray=" none" stroke-linecap=" butt" stroke-dashoffset="0" stroke-linejoin=" butt" stroke-miterlimit="4"/>
                        }
                    ).collect_view()
                )}
                // The actual note
                {
                    if up {
                        view!{<QuarterUp head_center_x={x+2. * SIGN_WIDTH + 15.} head_center_y=y index=index/>}
                    } else {
                        view!{<QuarterDown head_center_x={x+2. * SIGN_WIDTH + 15.} head_center_y=y index=index />}
                    }
                }

        </g>
    }
}

/// The size of one tact block, containig one note with signs.
const BLOCK_WIDTH: f32 = 50.;
/// The horizontal space to be allocated for the display of a single sign.
const SIGN_WIDTH: f32 = 10.;

/// Creates an svg view for an SATB block consisting of 4 notes.
#[component]
fn SatbBlockSvg(block: crate::logic::notes::SatbBlock, index: usize, x: f32) -> impl IntoView {
    let soprano_signs =
        if (block.0.get_note_line_and_sign().0 - block.1.get_note_line_and_sign().0).abs() < 6. {
            block.0.get_note_line_and_sign().1 % 0.75
        } else {
            0.
        };
    let tenor_signs =
        if (block.2.get_note_line_and_sign().0 - block.3.get_note_line_and_sign().0).abs() < 6. {
            block.2.get_note_line_and_sign().1 % 0.75
        } else {
            0.
        };

    view! {
        <g id={format!("SATB-Block {}", index)}>
        // Soprano and Alto note switched so alto note (which often has helper lines over the soprano note) is drawn earlier and thus the heler lines are below the soprano note.
            <Note index=index note=block.1 x=x move_to_lower_lines=false up=false id="Alto" sign_x={soprano_signs.abs()*2.*SIGN_WIDTH} />
            <Note index=index note=block.0 x=x move_to_lower_lines=false up=true id="Soprano"  />
            <Note index=index note=block.2 x=x move_to_lower_lines=true up=true id="Tenor" />
            <Note index=index note=block.3 x=x move_to_lower_lines=true up=false id="Bass" sign_x={tenor_signs.abs()*2.*SIGN_WIDTH} />
            <line id="left_line" vector-effect="non-scaling-stroke" x1={x+50.} y1="20.0" x2={x+50.} y2="180.0" stroke-width="1.5" stroke=" rgb(0,0,0)" stroke-dasharray=" none" stroke-linecap=" butt" stroke-dashoffset="0" stroke-linejoin=" butt" stroke-miterlimit="4" fill=" rgb(0,0,0)" fill-rule=" nonzero"/>
        </g>
    }
}

/// Creates an svg view for an entire result, consisting of a musical staff and multiple SATB blocks.
#[component]
pub fn ResultSvg(result: Vec<crate::logic::notes::SatbBlock>) -> impl IntoView {
    view! {
        <svg width={60+(BLOCK_WIDTH as usize)*result.len()+20} height=200 xmlns="http://www.w3.org/2000/svg">
            <super::svg::FullStaff width=(5+(BLOCK_WIDTH as usize)*result.len()) as f32/>
            {
                result.into_iter().enumerate().map(|(index, block)|
                    view!{
                        <super::svg::SatbBlockSvg block=block index=index x=60. + BLOCK_WIDTH * index as f32 />
                    }
                ).collect_view()
            }
        </svg>
    }
}
