import zio.{Scope, ZIO, ZIOAppArgs, ZIOAppDefault}


object UtubeTrackerApp extends ZIOAppDefault{
  override def run: ZIO[Any with ZIOAppArgs with Scope, Any, Any] = ZIO.logInfo("Hello World")
}
