modules = []

stage "Build" {
  steps {
    shell "echo pre" {}
    shell "ls" {
      label = "List project"
    }
    shell "echo test" {}
    dir "src" {
      shell "ls" {}
    }

    dir "/tmp" {
      shell "ls" {
        label = "List temp files"
      }
    }
  }
}
