def branchName = "${env.BRANCH_NAME}"

def version(path) {
    readFile("${path}/__version__").split("\n")[0]
}

node('jenkins-slave-ec2') {
    env.PATH = "/usr/local/bin:${env.HOME}/.cargo/bin:${env.PATH}"

    stage('Setup') {
        sh "rustup update"
        deleteDir()
        checkout scm
    }

    stage('Build') {
        sh "cargo build --all"
    }

    stage('Test') {
        sh "cargo test --all"
    }

    if(branchName == "master") {
        stage('Publish release') {
            def rootPath = pwd()
            sh """
            git remote rm origin
            git remote add origin 'git@github.com:snipsco/nlu-utils.git'
            git config --global user.email 'jenkins@snips.ai'
            git config --global user.name 'Jenkins'
            git tag ${version(rootPath)}
            git push --tags
            """
        }
    }
}
