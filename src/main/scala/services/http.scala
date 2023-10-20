package services

import zio.{Task, TaskLayer, ZLayer}

object http {


  object backend {
    //Todo Add conf for log :logResponseBody, logRequestBody, etc ....
    val layer: TaskLayer[SttpBackend[Task, ZioStreams]] = {
      ZLayer.scoped {
        HttpClientZioBackend
          .map(
            b =>
              Slf4jLoggingBackend(
                b,
                logResponseBody = true,
                logRequestBody = true,
                logRequestHeaders = true,
                sensitiveHeaders = Set.empty[String]
              )
          )
      }
    }

  }

}
