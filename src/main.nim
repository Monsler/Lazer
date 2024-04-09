import strutils, os, httpclient, json, base64

proc main() = 
  let text = "| Lazer | Lazurite package manager |"
  let equ = repeat("=", len(text))
  echo equ, "\n", text, "\n", equ
  var http = newHttpClient()
  let args = commandLineParams()
  if len(args) > 0:
    let cmd = args[0]
    if cmd == "init":
      echo "Enter project name:"
      let pname = readLine(stdin)
      if len(pname) > 0:
        echo "Project $# created" % pname
        createDir(pname)
        createDir("$#/src/lib" % pname)
        writeFile("$#/lazer.conf" % pname, "{}")
        writeFile("$#/src/main.lzr" % pname, "print(\"Hello, world!\")")
    elif cmd == "install":
      let name = args[1]
      let dest = args[2]
      if fileExists("$#/lazer.conf" % dest):
        if dirExists(dest) or fileExists(dest):
          var conf = parseFile("$#/lazer.conf" % dest)
          echo "Looking for $# library..." % name
          let content = http.getContent("https://lazer-repo-nim-default-rtdb.firebaseio.com/crates/$#.json" % name)
          if content != "null":
            let package = parseJson(content)
            echo "Found $# package [version: $#]" % [name, $package["version"]]
            conf[name] = %* package
            writeFile("$#/lazer.conf" % dest, $conf)
            echo "Saving $#.lzr to $#/src/lib/$#.lzr" % [name, dest, name]
            writeFile("$#/src/lib/$#.lzr" % [dest, name], decode($package{"package"}.getStr()))
        else:
          echo "Warning: Folder $# is not exist." % dest
      else:
        echo "Warning: The dest path do not contains lazer.conf"
    elif cmd == "remove":
      let name = args[1]
      let dest = args[2]
      if fileExists("$#/src/lib/$#.lzr" % [dest, name]):
        removeFile("$#/src/lib/$#.lzr" % [dest, name])
        let conf = parseFile("$#/lazer.conf" % dest)
        conf.delete(name)
        writeFile("$#/lazer.conf" % dest, $conf)
        echo "Library $# removed successfully." % name
      else:
        echo "Candidates for $# is not exists." % name
        echo "$#/src/lib/$#.lzr" % [dest, name]
  else:
    echo "Help:\n\binit: creates a new project.\n\binstall - installs declared module. syntax: install [name,] [project_path]\n\bremove - removing installed module. syntax: remove [name,] [dest]"

  http.close()

when isMainModule:
  main()
