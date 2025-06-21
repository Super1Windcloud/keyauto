set shell :=   ["pwsh.exe", "-NoProfile", "-c"]

github :
  git push github master:main
  git push github   v0.0.1



action :
  git add -A  && git commit -m "update" && git push origin master && git push github master:main
  git push github   v0.0.1