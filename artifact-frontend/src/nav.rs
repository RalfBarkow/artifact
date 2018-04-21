/* artifact: the requirements tracking tool made for developers
 * Copyright (C) 2018  Garrett Berg <@vitiral, vitiral@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the Lesser GNU General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the Lesser GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 * */

use dev_prelude::*;
use name;

pub(crate) fn view_nav(model: &Model, page: HtmlApp) -> HtmlApp {
    let search = &model.nav.search;
    let icon = if search.on {
        // Doesn't appear to work due to a bug?
        // css classes are not updated for some reason.
        FA_SEARCH_MINUS
    } else {
        FA_SEARCH
    };
    eprintln!("Search icon: {}", icon);
    html![<div>
        <div class=(CLEARFIX, MB2, ACE_WHITE, ACE_BG_BLACK, P1),>
            <button class=(BTN, REGULAR), id="search",
             onclick=|_| Msg::ToggleSearch,>
                <i class=(FA, FA_SEARCH),></i>
                <span>{ "Search" }</span>
            </button>
        </div>

        <div class=(CLEARFIX, PY1),>
            { search_pane(model) }
            <div class=(SM_COL, SM_COL_11, MD_COL_7, LG_COL_9),>
                { page }
            </div>
        </div>
    </div>]
}

pub(crate) fn search_pane(model: &Model) -> HtmlApp {
    if model.nav.search.on {
        let names = match Regex::new(&model.nav.search.value) {
            Ok(re) => html![<div>
                { for model.shared.artifacts
                    .keys()
                    .filter(|n| re.is_match(n.as_str()))
                    .map(|n| name::name_html(model, n))
                }
            </div>],
            Err(e) => html![
                <a
                 href="https://docs.rs/regex/0.2.10/regex/#syntax",
                 title="See syntax definition.",
                 class=(RED, BTN, BOLD),
                >
                { "INVALID REGEX" }
                </a>
            ],
        };
        html![<div class=(SM_COL, SM_COL_6, MD_COL_4, LG_COL_2, MR1),>
            <input
             id="search-input",
             value=model.nav.search.value.clone(),
             oninput=|e: InputData| Msg::SetSearch(e.value),
             class=INPUT,
             ></input>
            { names }
        </div>]
    } else {
        html![<div></div>]
    }
}