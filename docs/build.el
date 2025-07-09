(require 'ox-publish)

(setq org-publish-project-alist
      '(("Spacerobo"
         :base-directory "src"
         :publishing-function org-html-publish-to-html
         :publishing-directory "dist"
         :html-head-include-default-style nil

         ;; Stop to use sitemap generator
         :auto-sitemap n

         ;; CSS loader from HTTP
         :html-head "<link href=\"https://thomasf.github.io/solarized-css/solarized-dark.css\" rel=\"stylesheet\"></link>
         <link href=\"favicon.ico\" rel=\"icon\"></link>")))

(org-publish-all t)
