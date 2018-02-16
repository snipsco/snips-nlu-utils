@Library('snips@0.0') _

build = [
    stableBranch: "master",
    devBranch: "develop",
]

def builders = [:]
def branchName = "${env.BRANCH_NAME}"

def version(path) {
    readFile("${path}/__version__").split("\n")[0]
}

def uploadAsset(pythonPath, venvPath, asset="bdist_wheel") {
    env.PATH = "/usr/local/bin:${env.HOME}/.cargo/bin:${env.PATH}"

    def VIRTUALENV = "virtualenv -p $pythonPath $venvPath"
    def VENV = ". ${venvPath}/bin/activate"

    sh """
    cd python
    ${VIRTUALENV}
    ${VENV}
    pip install -r requirements.txt
    ssh-agent sh -c \" ssh-add; python setup.py ${asset} upload -r pypisnips \"
    """
}


def rustBuildAndTest(){
    env.PATH = "/usr/local/bin:${env.HOME}/.cargo/bin:${env.PATH}"
    checkout scm

    stage('Rust build') {
        sh "cargo build --all"
    }

    stage('Rust tests') {
        sh "cargo test --all"
    }
}

def pythonBuildAndTest(pythonPath, venvPath) {
    def VIRTUALENV = "virtualenv -p $pythonPath $venvPath"
    def VENV = ". ${venvPath}/bin/activate"

    stage('Setup') {
        deleteDir()
        checkout scm
        sh """
        cd python
        rm -rf ${venvPath}
        ${VIRTUALENV}
        ${VENV}
        pip install -r requirements.txt
        """
    }

    stage('Python build') {
        sh """
        cd python
        ${VENV}
        ssh-agent sh -c \" ssh-add; pip install -e . --verbose \"
        """
    }

    stage('Python tests') {
        sh """
        cd python
        ${VENV}
        python -m unittest discover
        """
    }
}


node('jenkins-slave-ec2') {

    stage('Build') {
        builders['linux-x86_64'] = {
            node('jenkins-slave-ec2') {
                checkout scm
                def id = sh(returnStdout: true, script: 'git rev-parse --short HEAD').trim()
                def python27path = sh(returnStdout: true, script: 'which python2.7').trim()
                def python34path = sh(returnStdout: true, script: 'which python3.4').trim()

                rustBuildAndTest()

                pythonBuildAndTest(python27path, "/tmp/venv27-$id-${env.EXECUTOR_NUMBER}")
                pythonBuildAndTest(python34path, "/tmp/venv34-$id-${env.EXECUTOR_NUMBER}")
            }
        }

        builders['apple-macos-compile'] = {
           node('macos') {
                rustBuildAndTest()

                def python27path = sh(returnStdout: true, script: 'which python2.7').trim()
                def python34path = sh(returnStdout: true, script: 'which python3.4').trim()
                def python35path = sh(returnStdout: true, script: 'which python3.5').trim()
                def python36path = sh(returnStdout: true, script: 'which python3.6').trim()

                pythonBuildAndTest(python27path, "venv27")
                pythonBuildAndTest(python34path, "venv34")
                pythonBuildAndTest(python35path, "venv35")
                pythonBuildAndTest(python36path, "venv36")
           }
        }

        parallel builders
    }

    if(branchName=="master") {
        stage('Upload assets') {
            builders['linux-x86_64'] = {
                node('jenkins-slave-ec2') {
                    def python27path = sh(returnStdout: true, script: 'which python2.7').trim()
                    def python34path = sh(returnStdout: true, script: 'which python3.4').trim()

                    uploadAsset(python27path, "venv27", "sdist")
                    uploadAsset(python27path, "venv27")
                    uploadAsset(python34path, "venv34")
                }
            }

            builders['apple-macos-compile'] = {
               node('macos') {
                    def python27path = sh(returnStdout: true, script: 'which python2.7').trim()
                    def python34path = sh(returnStdout: true, script: 'which python3.4').trim()
                    def python35path = sh(returnStdout: true, script: 'which python3.5').trim()
                    def python36path = sh(returnStdout: true, script: 'which python3.6').trim()

                    uploadAsset(python27path, "venv27")
                    uploadAsset(python34path, "venv34")
                    uploadAsset(python35path, "venv35")
                    uploadAsset(python36path, "venv36")
               }
            }

            parallel builders
        }
    }

    performReleaseIfNeeded()
}
