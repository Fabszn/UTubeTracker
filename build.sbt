import Dependencies._
import sbt.Keys.libraryDependencies



ThisBuild / scalaVersion := "3.3.1"

lazy val utubetracker = (project in file("."))
  .settings(
    libraryDependencies ++= zio,
    libraryDependencies ++= sttp,
    libraryDependencies += pureConfig

  )