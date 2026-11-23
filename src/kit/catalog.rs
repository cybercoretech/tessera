/// A documented Tessera class paired with its intended semantic HTML element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CatalogEntry {
    pub category: &'static str,
    pub class_name: &'static str,
    pub purpose: &'static str,
    pub semantic_element: &'static str,
}

/// Canonical classes currently included in the core Kit stylesheet.
pub const ENTRIES: &[CatalogEntry] = &[
    CatalogEntry {
        category: "Foundation",
        class_name: "ts-prose",
        purpose: "Constrain a reading column and trim its outer margins.",
        semantic_element: "article, section, or other content container",
    },
    CatalogEntry {
        category: "Foundation",
        class_name: "ts-sr-only",
        purpose: "Keep supporting text available to assistive technology.",
        semantic_element: "Any element containing supporting text",
    },
    CatalogEntry {
        category: "Layout",
        class_name: "ts-container",
        purpose: "Center content with a responsive maximum width and page gutter.",
        semantic_element: "Any page section or layout wrapper",
    },
    CatalogEntry {
        category: "Layout",
        class_name: "ts-section",
        purpose: "Apply responsive block spacing to a page section.",
        semantic_element: "section or main content region",
    },
    CatalogEntry {
        category: "Layout",
        class_name: "ts-grid",
        purpose: "Create an auto-fitting grid that adapts to available width.",
        semantic_element: "A wrapper around related items",
    },
    CatalogEntry {
        category: "Layout",
        class_name: "ts-stack",
        purpose: "Arrange related content vertically with a consistent gap.",
        semantic_element: "A wrapper around related content",
    },
    CatalogEntry {
        category: "Layout",
        class_name: "ts-cluster",
        purpose: "Group inline items that wrap naturally when space is limited.",
        semantic_element: "A wrapper around related actions or labels",
    },
    CatalogEntry {
        category: "Layout",
        class_name: "ts-sidebar-layout",
        purpose: "Pair a narrow supporting region with a flexible main region.",
        semantic_element: "A wrapper containing complementary and main content",
    },
    CatalogEntry {
        category: "Control",
        class_name: "ts-button",
        purpose: "Style a native button or link used for an action.",
        semantic_element: "button or a[href]",
    },
    CatalogEntry {
        category: "Control",
        class_name: "ts-button--secondary",
        purpose: "Give a secondary action the theme's secondary color.",
        semantic_element: "button or a[href]",
    },
    CatalogEntry {
        category: "Control",
        class_name: "ts-label",
        purpose: "Give a persistent label clear visual hierarchy.",
        semantic_element: "label associated with a form control",
    },
    CatalogEntry {
        category: "Control",
        class_name: "ts-input",
        purpose: "Style a native text, email, search, or numeric input.",
        semantic_element: "input",
    },
    CatalogEntry {
        category: "Control",
        class_name: "ts-select",
        purpose: "Style a native option selector while retaining its platform behavior.",
        semantic_element: "select with a visible label",
    },
    CatalogEntry {
        category: "Control",
        class_name: "ts-textarea",
        purpose: "Style a resizable native multiline text control.",
        semantic_element: "textarea with a visible label",
    },
    CatalogEntry {
        category: "Control",
        class_name: "ts-field",
        purpose: "Group a label, control, help text, and optional error message.",
        semantic_element: "div or field container with associated labels and descriptions",
    },
    CatalogEntry {
        category: "Control",
        class_name: "ts-choice-group",
        purpose: "Group related native checkboxes or radio buttons under a legend.",
        semantic_element: "fieldset with legend and labelled checkbox/radio inputs",
    },
    CatalogEntry {
        category: "Control",
        class_name: "ts-choice",
        purpose: "Lay out an individual native checkbox or radio option with its label.",
        semantic_element: "label containing a checkbox or radio input",
    },
    CatalogEntry {
        category: "Control",
        class_name: "ts-switch",
        purpose: "Style a native checkbox as an on/off setting control.",
        semantic_element: "label containing input type=checkbox with role=switch",
    },
    CatalogEntry {
        category: "Feedback",
        class_name: "ts-error",
        purpose: "Show a textual form error associated through aria-describedby.",
        semantic_element: "p or span referenced by aria-describedby",
    },
    CatalogEntry {
        category: "Control",
        class_name: "ts-help",
        purpose: "Style short instructions associated with a form control.",
        semantic_element: "p or span referenced by aria-describedby",
    },
    CatalogEntry {
        category: "Surface",
        class_name: "ts-card",
        purpose: "Group related content on a distinct surface.",
        semantic_element: "article, section, or div as appropriate to the content",
    },
    CatalogEntry {
        category: "Content",
        class_name: "ts-badge",
        purpose: "Present a short status or category label.",
        semantic_element: "span",
    },
    CatalogEntry {
        category: "Feedback",
        class_name: "ts-alert",
        purpose: "Present a concise status or notice within the page flow.",
        semantic_element: "div, with role=status or role=alert when appropriate",
    },
    CatalogEntry {
        category: "Navigation",
        class_name: "ts-nav",
        purpose: "Lay out a navigation region's brand and links.",
        semantic_element: "nav with an accessible name",
    },
    CatalogEntry {
        category: "Navigation",
        class_name: "ts-nav__links",
        purpose: "Lay out navigation links as a wrapping list.",
        semantic_element: "ul containing li and a elements",
    },
    CatalogEntry {
        category: "Navigation",
        class_name: "ts-breadcrumbs",
        purpose: "Show the current page's location within a hierarchy.",
        semantic_element: "nav with an ordered list and aria-current on the current page",
    },
    CatalogEntry {
        category: "Navigation",
        class_name: "ts-pagination",
        purpose: "Navigate between related pages or result sets.",
        semantic_element: "nav with a list of links and aria-current on the current page",
    },
    CatalogEntry {
        category: "Content",
        class_name: "ts-avatar",
        purpose: "Show a person's image or labelled initials in a consistent circle.",
        semantic_element: "img with useful alt text, or a labelled element for initials",
    },
    CatalogEntry {
        category: "Content",
        class_name: "ts-media",
        purpose: "Pair a visual or avatar with flexible descriptive content.",
        semantic_element: "figure with figcaption, or a wrapper with meaningful children",
    },
    CatalogEntry {
        category: "Data",
        class_name: "ts-table-wrap",
        purpose: "Keep wide tables scrollable within their own region.",
        semantic_element: "div containing a native table",
    },
    CatalogEntry {
        category: "Data",
        class_name: "ts-table",
        purpose: "Style a data table while retaining native row and header semantics.",
        semantic_element: "table with caption, thead, tbody, and scoped headers",
    },
    CatalogEntry {
        category: "Disclosure",
        class_name: "ts-accordion",
        purpose: "Group expandable answers using the browser's native disclosure behavior.",
        semantic_element: "div containing details, summary, and panel content",
    },
];
