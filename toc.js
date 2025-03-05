// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="intro.html"><strong aria-hidden="true">1.</strong> Intro</a></li><li class="chapter-item expanded affix "><li class="part-title">for Users</li><li class="chapter-item expanded "><a href="users/getting-started.html"><strong aria-hidden="true">2.</strong> Getting Started</a></li><li class="chapter-item expanded "><a href="users/feed.html"><strong aria-hidden="true">3.</strong> Feed</a></li><li class="chapter-item expanded "><a href="users/redirecting.html"><strong aria-hidden="true">4.</strong> Redirecting</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="users/redirecting-with-static-files-and-markup.html"><strong aria-hidden="true">4.1.</strong> with Static files and Markup</a></li><li class="chapter-item expanded "><a href="users/redirecting-with-redirects-file.html"><strong aria-hidden="true">4.2.</strong> with Redirects file</a></li><li class="chapter-item expanded "><a href="users/redirecting-with-platform-specific-config.html"><strong aria-hidden="true">4.3.</strong> with Platform-Specific Configuration</a></li><li class="chapter-item expanded "><a href="users/redirecting-with-aoba.html"><strong aria-hidden="true">4.4.</strong> with Aoba (Lume &amp; Hono)</a></li><li class="chapter-item expanded "><a href="users/redirecting-with-fep-612d.html"><strong aria-hidden="true">4.5.</strong> with FEP-612d</a></li></ol></li><li class="chapter-item expanded "><a href="users/backfeed.html"><strong aria-hidden="true">5.</strong> Backfeed</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="users/backfeed-based-on-kkna.html"><strong aria-hidden="true">5.1.</strong> based on KKna</a></li><li class="chapter-item expanded "><a href="users/backfeed-based-on-mastodon-comments.html"><strong aria-hidden="true">5.2.</strong> based on Mastodon Comments</a></li><li class="chapter-item expanded "><a href="users/backfeed-based-on-webmention.html"><strong aria-hidden="true">5.3.</strong> based on Webmention (TODO)</a></li></ol></li><li class="chapter-item expanded "><li class="part-title">for Admins</li><li class="chapter-item expanded "><a href="admins/install.html"><strong aria-hidden="true">6.</strong> Install</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="admins/install-docker.html"><strong aria-hidden="true">6.1.</strong> Docker Installation</a></li><li class="chapter-item expanded "><a href="admins/install-binary.html"><strong aria-hidden="true">6.2.</strong> Binary Installation</a></li><li class="chapter-item expanded "><a href="admins/install-nix.html"><strong aria-hidden="true">6.3.</strong> Nix/NixOS Installation</a></li></ol></li><li class="chapter-item expanded "><a href="admins/environments.html"><strong aria-hidden="true">7.</strong> Environments</a></li><li class="chapter-item expanded "><a href="admins/create-account.html"><strong aria-hidden="true">8.</strong> Create Account</a></li><li class="chapter-item expanded "><a href="admins/block-instances-or-actors.html"><strong aria-hidden="true">9.</strong> Block Instances or Actors</a></li><li class="chapter-item expanded affix "><li class="part-title">for Developers</li><li class="chapter-item expanded "><a href="developers/prepare.html"><strong aria-hidden="true">10.</strong> Prepare</a></li><li class="chapter-item expanded "><a href="developers/development-local.html"><strong aria-hidden="true">11.</strong> Local Development</a></li><li class="chapter-item expanded "><a href="developers/development-docker.html"><strong aria-hidden="true">12.</strong> Docker Development</a></li><li class="chapter-item expanded affix "><li class="part-title">Others</li><li class="chapter-item expanded "><a href="others/compatibility-chart.html"><strong aria-hidden="true">13.</strong> Compatibility Chart</a></li><li class="chapter-item expanded "><a href="others/federation.html"><strong aria-hidden="true">14.</strong> Federation</a></li><li class="chapter-item expanded "><a href="others/json-feed-extension.html"><strong aria-hidden="true">15.</strong> JSON Feed Extension</a></li><li class="chapter-item expanded "><a href="others/packaging-status.html"><strong aria-hidden="true">16.</strong> Packaging Status</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
